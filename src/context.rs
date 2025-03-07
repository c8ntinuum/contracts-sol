use anchor_lang::{accounts::interface_account::InterfaceAccount, prelude::*};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount, TokenInterface};
use crate::ConfigAccount;
use crate::CONFIG_SEED;
use crate::VAULT_ACCOUNT_SEED;
use crate::SWAP_PROGRAM_ID;
use crate::USDT_WSOL_POOL_ID;
use crate::USDT_WSOL_POOL_VAULT_WSOL;
use crate::USDT_WSOL_POOL_VAULT_USDT;

use crate::C8NT_WSOL_POOL_ID;
use crate::C8NT_WSOL_POOL_AMM_CONFIG;
use crate::C8NT_WSOL_POOL_LP_MINT;
use crate::C8NT_WSOL_POOL_VAULT_WSOL;
use crate::C8NT_WSOL_POOL_VAULT_C8NT;
use crate::C8NT_WSOL_POOL_OBSERVATION_STATE;
use crate::C8NT_WSOL_POOL_AUTHORITY;

use crate::WSOL_MINT;
use crate::C8NT_MINT;

use crate::CustomError;

#[derive(Accounts)]
pub struct Setting<'info> {
    #[account(
      mut,
      seeds = [CONFIG_SEED.as_bytes()],
      bump,
    )]
    pub config_account: Account<'info, ConfigAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SettingWithSystemProgram<'info> {
    #[account(
      mut,
      seeds = [CONFIG_SEED.as_bytes()],
      bump,
    )]
    pub config_account: Account<'info, ConfigAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
      init,
      space = ConfigAccount::ACCOUNT_SIZE,
      payer = signer,
      seeds = [CONFIG_SEED.as_bytes()],
      bump,
    )]
    pub config_account: Account<'info, ConfigAccount>,

    #[account(
    mut,
    seeds = [VAULT_ACCOUNT_SEED.as_bytes()],  // Seed used to derive the PDA
    bump,
  )]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


// Context for the generate function
#[derive(Accounts)]
pub struct Generate<'info> {
    #[account(
      seeds = [CONFIG_SEED.as_bytes()],
      bump,
    )]
    pub config_account: Account<'info, ConfigAccount>,

    // USDT - WSOL pool details
    #[account(address = SWAP_PROGRAM_ID)]
    pub raydium_swap_program: UncheckedAccount<'info>,

    #[account(address = USDT_WSOL_POOL_ID)]
    pub pool_usdt_wsol: UncheckedAccount<'info>,

    #[account(address = USDT_WSOL_POOL_VAULT_WSOL)]
    pub pool_usdt_wsol_token_wsol_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(address = USDT_WSOL_POOL_VAULT_USDT)]
    pub pool_usdt_wsol_token_usdt_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    // C8NTM - WSOL pool details
    #[account(address = C8NT_WSOL_POOL_AMM_CONFIG)]
    pub pool_amm_config: UncheckedAccount<'info>,

    #[account(address = C8NT_WSOL_POOL_AUTHORITY)]
    pub pool_authority: UncheckedAccount<'info>,

    #[account(address = WSOL_MINT)]
    pub wsol_mint: Box<InterfaceAccount<'info, Mint>>,
    
    #[account(mut, address = C8NT_MINT)]
    pub c8nt_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, address = C8NT_WSOL_POOL_ID)]
    pub pool_state: UncheckedAccount<'info>,   

    #[account(mut, address = C8NT_WSOL_POOL_LP_MINT)]
    pub pool_lp_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, address = C8NT_WSOL_POOL_VAULT_WSOL)]
    pub pool_wsol_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut, address = C8NT_WSOL_POOL_VAULT_C8NT)]
    pub pool_c8nt_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut, address = C8NT_WSOL_POOL_OBSERVATION_STATE)]
    pub pool_observation_state: UncheckedAccount<'info>, 

    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED.as_bytes()], 
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub vault_wsol_token_account: Box<InterfaceAccount<'info, TokenAccount>>, //a.k.a. input/output_token_account or token_X_account

    #[account(mut)]
    pub vault_c8nt_token_account: Box<InterfaceAccount<'info, TokenAccount>>, //a.k.a. input/output_token_account or token_X_account

    #[account(mut)]
    pub vault_lp_token_account: Box<InterfaceAccount<'info, TokenAccount>>, //a.k.a. owner_lp_token

    #[account(
        mut,
        constraint = referral1.key() != Pubkey::default() @ CustomError::InvalidReferral
    )]
    pub referral1: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = referral1.key() != referral2.key() @ CustomError::InvalidReferral2
    )]
    pub referral2: UncheckedAccount<'info>,

    // Common accounts
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub token_program_2022: Program<'info, Token2022>,
}