//src/instructions/generate.rs
use crate::*;
use solana_program::hash::hash;

pub fn generate_handler<'info>(
    ctx: Context<Generate<'info>>,
    sol_amount: u64,            //
    ctnm_usd_price_oracle: u64, //
    deadline: i64,              //
    slippage: u16,              //
    signature: [u8; 64],        //
) -> Result<()> {
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;

    // Validate input
    require!(ctx.accounts.config_account.is_paused == false, CustomError::ContractIsPaused);
    require!(deadline >= timestamp, CustomError::DeadlinePassed);
    require!(slippage as u64 <= PERCENTAGE_DIVIDER / 5, CustomError::WrongSlippage);
    require!(sol_amount >= GENERATION_MIN_AMOUNT, CustomError::ValueTooLow);
    require!(
        !ctx.accounts.config_account.black_list.contains(&ctx.accounts.signer.to_account_info().key()),
        CustomError::UserIsBlackListed
    );
    require!(ctx.accounts.signer.to_account_info().lamports() >= sol_amount, CustomError::InsufficientBalance);
    require!(ctx.accounts.signer.to_account_info().executable == false, CustomError::SenderNotEOA);
    require!(ctnm_usd_price_oracle > 0, CustomError::InvalidPriceOracle);
    require!(
        ctx.accounts.withdraw_to.to_account_info().key() == ctx.accounts.config_account.withdrawal_address,
        CustomError::UnauthorizedWithdrawalAddress
    );

    // Build message hash based on input parameters
    let message = [
        &ctx.program_id.as_ref(),
        C8NT_MINT.as_ref(),
        &ctnm_usd_price_oracle.to_le_bytes(),
        &deadline.to_le_bytes(),
        ctx.accounts.referral1.key().as_ref(),
        ctx.accounts.referral2.key().as_ref(),
        ctx.accounts.signer.key().as_ref(),
    ]
    .concat();
    let message_hash = hash(&message).to_bytes();

    // Validate input parameters against signature
    let is_valid_signature = solana_ed25519_verify::verify_signature(&ctx.accounts.config_account.verifier, &signature, &message_hash).map_err(|_| error!(CustomError::WrongSignature))?;
    require!(is_valid_signature, CustomError::WrongSignature);

    // Get ratio of 1 c8nt in wsol
    let (_, ratio_1_c8nt_in_wsol) = utils::token_price(ctx.accounts.pool_wsol_vault.amount, ctx.accounts.pool_c8nt_vault.amount);

    // Get ratio of 1 wsol in usdt
    let pool_state_data: Vec<u8> = ctx.accounts.pool_usdt_wsol.try_borrow_data()?.to_vec();
    let pool_state_bytes: &[u8] = &pool_state_data[8..];
    let pool_state: &PoolState = bytemuck::from_bytes(pool_state_bytes);
    let ratio_1_wsol_in_usdt: f64 = tick_to_price(pool_state.tick_current);

    // Calculate C8NT price in USDT
    let mut ctnm_usd_price_pool: u64 = (ratio_1_c8nt_in_wsol * ratio_1_wsol_in_usdt * (1_000_000_000 as f64)) as u64;
    if ctnm_usd_price_pool < ctx.accounts.config_account.global_generation_price_usd {
        ctnm_usd_price_pool = ctx.accounts.config_account.global_generation_price_usd
    }

    // Validate slippage
    require!(
        ctnm_usd_price_pool <= (ctnm_usd_price_oracle * (slippage as u64 + PERCENTAGE_DIVIDER) / PERCENTAGE_DIVIDER),
        CustomError::InvalidPriceOraclePriceHigher
    );
    require!(
        (ctnm_usd_price_oracle * (PERCENTAGE_DIVIDER - slippage as u64) / PERCENTAGE_DIVIDER) <= ctnm_usd_price_pool,
        CustomError::InvalidPriceOraclePriceLower
    );

    // Take solana from signer
    transfer_sol(
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.signer.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        sol_amount,
    )?;

    // Calculate deposit distribution
    let liquidity_amount: u64 = sol_amount * ctx.accounts.config_account.liquidity_percentage / PERCENTAGE_DIVIDER;

    // Init vault pda seeds user to sign vault transfers
    let seeds: &[&[u8]; 2] = &[VAULT_ACCOUNT_SEED.as_bytes(), &[ctx.bumps.vault]];
    let vault_seeds: &[&[&[u8]]] = &[seeds];

    // Wrap WSOL for both operations
    wrap_wsol(
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.vault_wsol_token_account.to_account_info(),
        vault_seeds,
        liquidity_amount,
    )?;

    // Swap WSOL / C8NTM
    swap_wsol_4_c8nt(
        ctx.accounts.raydium_swap_program.to_account_info(),
        liquidity_amount / 2,
        0, // allow any C8NT amount, we do not care since we have a validation on pool prices
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.pool_authority.to_account_info(),
        ctx.accounts.pool_amm_config.to_account_info(),
        ctx.accounts.pool_state.to_account_info(),
        ctx.accounts.vault_wsol_token_account.to_account_info(),
        ctx.accounts.vault_c8nt_token_account.to_account_info(),
        ctx.accounts.pool_wsol_vault.to_account_info(),
        ctx.accounts.pool_c8nt_vault.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.wsol_mint.to_account_info(),
        ctx.accounts.c8nt_mint.to_account_info(),
        ctx.accounts.pool_observation_state.to_account_info(),
        vault_seeds,
    )?;

    // Reload accounts
    ctx.accounts.pool_c8nt_vault.reload()?;
    ctx.accounts.pool_wsol_vault.reload()?;
    ctx.accounts.vault_wsol_token_account.reload()?;
    ctx.accounts.vault_c8nt_token_account.reload()?;

    // Calculate LP tokens to receive
    let lp_token_amount: u64 = if ctx.accounts.pool_lp_mint.supply == 0 {
        (ctx.accounts.vault_wsol_token_account.amount as f64 * ctx.accounts.vault_c8nt_token_account.amount as f64).sqrt() as u64
    } else {
        std::cmp::min(
            (ctx.accounts.vault_wsol_token_account.amount as u128 * ctx.accounts.pool_lp_mint.supply as u128 / ctx.accounts.pool_wsol_vault.amount as u128) as u64,
            (ctx.accounts.vault_c8nt_token_account.amount as u128 * ctx.accounts.pool_lp_mint.supply as u128 / ctx.accounts.pool_c8nt_vault.amount as u128) as u64,
        )
    };

    // Add Liquidity  WSOL / C8NTM
    add_liquidity_wsol_c8nt(
        ctx.accounts.raydium_swap_program.to_account_info(),
        lp_token_amount,
        ctx.accounts.vault_wsol_token_account.amount,
        ctx.accounts.vault_c8nt_token_account.amount,
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.pool_authority.to_account_info(),
        ctx.accounts.pool_state.to_account_info(),
        ctx.accounts.vault_lp_token_account.to_account_info(),
        ctx.accounts.vault_wsol_token_account.to_account_info(),
        ctx.accounts.vault_c8nt_token_account.to_account_info(),
        ctx.accounts.pool_wsol_vault.to_account_info(),
        ctx.accounts.pool_c8nt_vault.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token_program_2022.to_account_info(),
        ctx.accounts.wsol_mint.to_account_info(),
        ctx.accounts.c8nt_mint.to_account_info(),
        ctx.accounts.pool_lp_mint.to_account_info(),
        vault_seeds,
    )?;

    // Deal with referrals
    let mut main_referral_amount: u64 = sol_amount * ctx.accounts.config_account.main_referral_percentage / PERCENTAGE_DIVIDER;
    let mut second_referral_amount: u64 = sol_amount * ctx.accounts.config_account.second_referral_percentage / PERCENTAGE_DIVIDER;
    if ctx.accounts.referral2.to_account_info().key() == NO_REFERRAL {
        main_referral_amount += second_referral_amount;
        second_referral_amount = 0;
    }

    if main_referral_amount > 0 {
        transfer_sol_from_pda(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.referral1.to_account_info(),
            main_referral_amount,
            vault_seeds,
        )?;
    }
    if second_referral_amount > 0 {
        transfer_sol_from_pda(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.referral2.to_account_info(),
            second_referral_amount,
            vault_seeds,
        )?;
    }

    // Burn all C8NTM remained after add liquidity
    ctx.accounts.vault_c8nt_token_account.reload()?;
    if ctx.accounts.vault_c8nt_token_account.amount > 0 {
        burn_tokens_from_vault(
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.c8nt_mint.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.vault_c8nt_token_account.to_account_info(),
            ctx.accounts.vault_c8nt_token_account.amount,
            vault_seeds,
        )?;
    }

    // Burn LP tokens
    ctx.accounts.vault_lp_token_account.reload()?;
    if ctx.accounts.vault_lp_token_account.amount > 0 {
        burn_tokens_from_vault(
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.pool_lp_mint.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.vault_lp_token_account.to_account_info(),
            ctx.accounts.vault_lp_token_account.amount,
            vault_seeds,
        )?;
    }

    // Withdraw collected sol amounts from vault
    if ctx.accounts.vault.to_account_info().lamports() > 5_000_000 {
        transfer_sol_from_pda(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.withdraw_to.to_account_info(),
            ctx.accounts.vault.to_account_info().lamports() - 5_000_000,
            vault_seeds,
        )?;
    }

    emit!(GenerationCreated {
        signer: ctx.accounts.signer.to_account_info().key(),
        sol_amount: sol_amount,
        sol_amount_in_usdt: ((ratio_1_wsol_in_usdt * (sol_amount as f64)) as u64),
        ctnm_usd_price_oracle: ctnm_usd_price_oracle,
        ctnm_usd_price_pool: ctnm_usd_price_pool,
        slippage: slippage,
        referral1: ctx.accounts.referral1.key(),
        referral2: ctx.accounts.referral2.key(),
        signature: hex::encode(signature),
    });
    Ok(())
}
