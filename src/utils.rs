//src/utils.rs
use anchor_lang::prelude::AccountInfo;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::CpiContext;
use anchor_lang::prelude::ProgramError;
use anchor_lang::system_program;
use anchor_lang::Key;
use anchor_spl::token::Burn;
use solana_program::instruction::Instruction;
use solana_program::program::invoke;
use solana_program::program::invoke_signed;

pub const Q32: u128 = (u32::MAX as u128) + 1; // 2^32
pub const Q_RATIO: f64 = 1.0001;

pub fn tick_to_price(tick: i32) -> f64 {
    Q_RATIO.powi(tick)
}

pub fn token_price(
    token_0_amount: u64, //
    token_1_amount: u64, //
) -> (f64, f64) {
    let token_0_amount_x32: u128 = token_1_amount as u128 * Q32 as u128 / token_0_amount as u128;
    let token_1_amount_x32: u128 = token_0_amount as u128 * Q32 as u128 / token_1_amount as u128;
    return (token_0_amount_x32 as f64 / (2.0_f64.powf(32.0)), token_1_amount_x32 as f64 / (2.0_f64.powf(32.0)));
}

pub fn wrap_wsol<'info>(
    system_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    signer: AccountInfo<'info>,
    signer_wsol_token_account: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<(), ProgramError> {
    // Transfer SOL to WSOL token account
    system_program::transfer(
        CpiContext::new_with_signer(
            system_program,
            system_program::Transfer {
                from: signer.clone(),
                to: signer_wsol_token_account.clone(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    // Sync WSOL account after deposit
    let sync_ix = Instruction {
        program_id: token_program.key(),
        accounts: vec![AccountMeta::new(signer_wsol_token_account.key(), false)],
        data: vec![17],
    };
    invoke(&sync_ix, &[signer_wsol_token_account, token_program])?;
    Ok(())
}

pub fn swap_wsol_4_c8nt<'info>(
    raydium_swap_program: AccountInfo<'info>,     //
    wsol_amount_in: u64,                          //
    c8nt_amount_out_min: u64,                     //
    signer: AccountInfo<'info>,                   //
    pool_authority: AccountInfo<'info>,           //
    pool_amm_config: AccountInfo<'info>,          //
    pool_state: AccountInfo<'info>,               //
    vault_wsol_token_account: AccountInfo<'info>, //
    vault_c8nt_token_account: AccountInfo<'info>, //
    pool_wsol_vault: AccountInfo<'info>,          //
    pool_c8nt_vault: AccountInfo<'info>,          //
    wsol_token_program: AccountInfo<'info>,       //
    c8nt_token_program: AccountInfo<'info>,       //
    wsol_mint: AccountInfo<'info>,                //
    c8nt_mint: AccountInfo<'info>,                //
    pool_observation_state: AccountInfo<'info>,   //
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&[0x8f, 0xbe, 0x5a, 0xda, 0xc4, 0x1e, 0x33, 0xde]); //8fbe5adac41e33de //echo -n "global:swap_base_input" | shasum -a 256 | head -c 16
    instruction_data.extend_from_slice(&wsol_amount_in.to_le_bytes());
    instruction_data.extend_from_slice(&c8nt_amount_out_min.to_le_bytes());
    let swap_accounts = vec![
        AccountMeta::new(signer.key(), true),                       //
        AccountMeta::new_readonly(pool_authority.key(), false),     //authority
        AccountMeta::new_readonly(pool_amm_config.key(), false),    //amm_config
        AccountMeta::new(pool_state.key(), false),                  //pool_state
        AccountMeta::new(vault_wsol_token_account.key(), false),    //input_token_account
        AccountMeta::new(vault_c8nt_token_account.key(), false),    //output_token_account
        AccountMeta::new(pool_wsol_vault.key(), false),             //input_vault
        AccountMeta::new(pool_c8nt_vault.key(), false),             //output_vault
        AccountMeta::new_readonly(wsol_token_program.key(), false), //input_token_program
        AccountMeta::new_readonly(c8nt_token_program.key(), false), //output_token_program
        AccountMeta::new_readonly(wsol_mint.key(), false),          //input_token_mint
        AccountMeta::new_readonly(c8nt_mint.key(), false),          //output_token_mint
        AccountMeta::new(pool_observation_state.key(), false),      //observation_state
    ];
    let swap_instruction = Instruction {
        program_id: raydium_swap_program.key(),
        accounts: swap_accounts,
        data: instruction_data,
    };
    let acc_infos: &[AccountInfo<'_>; 14] = &[
        signer,                   //
        pool_authority,           //authority
        pool_amm_config,          //amm_config ??maybe read only
        pool_state,               //pool_state
        vault_wsol_token_account, //input_token_account
        vault_c8nt_token_account, //output_token_account
        pool_wsol_vault,          //input_vault
        pool_c8nt_vault,          //output_vault
        wsol_token_program,       //input_token_program
        c8nt_token_program,       //output_token_program
        wsol_mint,                //input_token_mint
        c8nt_mint,                //output_token_mint
        pool_observation_state,   //observation_state
        raydium_swap_program,
    ];
    invoke_signed(&swap_instruction, acc_infos, signer_seeds)?;
    Ok(())
}

pub fn add_liquidity_wsol_c8nt<'info>(
    raydium_swap_program: AccountInfo<'info>, //
    lp_token_amount: u64,
    deposit_amount_wsol_max: u64,
    deposit_amount_c8nt_max: u64,
    signer: AccountInfo<'info>,                   //
    pool_authority: AccountInfo<'info>,           //
    pool_state: AccountInfo<'info>,               //
    vault_lp_token_account: AccountInfo<'info>,   //
    vault_wsol_token_account: AccountInfo<'info>, //
    vault_c8nt_token_account: AccountInfo<'info>, //
    pool_wsol_vault: AccountInfo<'info>,          //
    pool_c8nt_vault: AccountInfo<'info>,          //
    token_program: AccountInfo<'info>,            //
    token_program_2022: AccountInfo<'info>,       //
    wsol_mint: AccountInfo<'info>,                //
    c8nt_mint: AccountInfo<'info>,                //
    pool_lp_mint: AccountInfo<'info>,             //
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let mut instruction_data_deposit = Vec::new();
    instruction_data_deposit.extend_from_slice(&[0xf2, 0x23, 0xc6, 0x89, 0x52, 0xe1, 0xf2, 0xb6]); //f223c68952e1f2b6 //echo -n "global:deposit" | shasum -a 256 | head -c 16
    instruction_data_deposit.extend_from_slice(&lp_token_amount.to_le_bytes());
    instruction_data_deposit.extend_from_slice(&deposit_amount_wsol_max.to_le_bytes());
    instruction_data_deposit.extend_from_slice(&deposit_amount_c8nt_max.to_le_bytes());
    let deposit_accounts = vec![
        AccountMeta::new(signer.key(), true),                       //
        AccountMeta::new_readonly(pool_authority.key(), false),     //authority
        AccountMeta::new(pool_state.key(), false),                  //pool_state
        AccountMeta::new(vault_lp_token_account.key(), false),      //owner_lp_token
        AccountMeta::new(vault_wsol_token_account.key(), false),    //token_0_account
        AccountMeta::new(vault_c8nt_token_account.key(), false),    //token_1_account
        AccountMeta::new(pool_wsol_vault.key(), false),             //token_0_vault
        AccountMeta::new(pool_c8nt_vault.key(), false),             //token_1_vault
        AccountMeta::new_readonly(token_program.key(), false),      //token_program
        AccountMeta::new_readonly(token_program_2022.key(), false), //token_program_2022
        AccountMeta::new_readonly(wsol_mint.key(), false),          //vault_0_mint
        AccountMeta::new_readonly(c8nt_mint.key(), false),          //vault_1_mint
        AccountMeta::new(pool_lp_mint.key(), false),                //lp_mint
    ];
    let deposit_instruction = Instruction {
        program_id: raydium_swap_program.key(),
        accounts: deposit_accounts,
        data: instruction_data_deposit, // Encode arguments
    };
    let acc_infos_deposit: &[AccountInfo<'_>; 14] = &[
        signer,                   //
        pool_authority,           //authority
        pool_state,               //pool_state
        vault_lp_token_account,   //owner_lp_token
        vault_wsol_token_account, //token_0_account
        vault_c8nt_token_account, //token_1_account
        pool_wsol_vault,          //token_0_vault
        pool_c8nt_vault,          //token_1_vault
        token_program,            //token_program
        token_program_2022,       //token_program_2022
        wsol_mint,                //vault_0_mint
        c8nt_mint,                //vault_1_mint
        pool_lp_mint,             //lp_mint
        raydium_swap_program,
    ];
    invoke_signed(&deposit_instruction, acc_infos_deposit, signer_seeds)?;
    Ok(())
}

pub fn transfer_sol<'info>(
    system_program: AccountInfo<'info>, //
    from: AccountInfo<'info>,           //
    to: AccountInfo<'info>,             //
    amount: u64,
) -> Result<(), ProgramError> {
    system_program::transfer(CpiContext::new(system_program, system_program::Transfer { from: from, to: to }), amount)?;
    Ok(())
}

pub fn transfer_sol_from_pda<'info>(
    system_program: AccountInfo<'info>, //
    from: AccountInfo<'info>,           //
    to: AccountInfo<'info>,             //
    amount: u64,                        //
    signer_seeds: &[&[&[u8]]],          //
) -> Result<(), ProgramError> {
    system_program::transfer(CpiContext::new_with_signer(system_program, system_program::Transfer { from: from, to: to }, signer_seeds), amount)?;
    Ok(())
}

pub fn burn_c8nt_from_pda<'info>(
    token_program: AccountInfo<'info>,       //
    c8nt_mint: AccountInfo<'info>,           //
    vault_account: AccountInfo<'info>,       //
    vault_token_account: AccountInfo<'info>, //
    amount: u64,                             //
    signer_seeds: &[&[&[u8]]],               //
) -> Result<(), ProgramError> {
    let cpi_accounts = Burn {
        mint: c8nt_mint,
        from: vault_token_account,
        authority: vault_account,
    };
    let cpi_ctx: CpiContext<'_, '_, '_, '_, Burn<'_>> = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);
    anchor_spl::token::burn(cpi_ctx, amount)?;
    Ok(())
}
