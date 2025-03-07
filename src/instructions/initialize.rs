// programs/num-token/src/instructions/initialize.rs
use crate::*;

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    // Init config account
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.admin = ctx.accounts.signer.key();
    config_account.verifier = SERVER_SIGNER_PUB;
    config_account.liquidity_percentage = PERCENTAGE_TO_LIQUIDITY;
    config_account.main_referral_percentage = PERCENTAGE_MAIN_REFERRAL;
    config_account.second_referral_percentage = PERCENTAGE_SECOND_REFERRAL;
    config_account.global_generation_price_usd = MIN_FLOOR_GENERATION_PRICE_USD;
    config_account.is_paused = false;
    config_account.black_list = Vec::with_capacity(BLACKLIST_LOT_SIZE);

    // Transfer some amount to vault to create it
    transfer_sol(
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.signer.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        5_000_000,
    )?;

    // Mark down the initial configuration
    msg!("Config Account: {}", config_account.key());
    msg!("Admin: {}", config_account.admin.key());
    msg!("Vault: {}", ctx.accounts.vault.key());
    msg!("Verifier: {}", config_account.verifier.key());
    msg!("Liquidity Percentage: {}", config_account.liquidity_percentage);
    msg!("Main Referral Percentage: {}", config_account.main_referral_percentage);
    msg!("Second Referral Percentage: {}", config_account.second_referral_percentage);
    msg!("Global Generation Price USDT: {}", config_account.global_generation_price_usd);
    msg!("Is Paused: {}", config_account.is_paused);
    msg!("Black List Length: {}", config_account.black_list.len());

    Ok(())
}
