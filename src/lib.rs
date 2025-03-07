// programs/num-token/src/lib.rs
use anchor_lang::prelude::*;

pub mod constants;
pub mod context;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

pub use constants::*;
pub use context::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("HYbDZYbzfEYdYBN7RSPZ3Y4s2KKmPB5kC7F2X6Vvq4EJ");

#[program]
pub mod c8ntinuum {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn generate<'info>(ctx: Context<Generate<'info>>, sol_amount: u64, ctnm_usd_price_oracle: u64, deadline: i64, slippage: u16, signature: [u8; 64]) -> Result<()> {
        generate_handler(ctx, sol_amount, ctnm_usd_price_oracle, deadline, slippage, signature)
    }

    pub fn add_to_black_list<'info>(ctx: Context<SettingWithSystemProgram<'info>>, address: Pubkey) -> Result<()> {
        add_to_black_list_handler(ctx, address)
    }

    pub fn set_verifier<'info>(ctx: Context<Setting<'info>>, new_verifier: Pubkey) -> Result<()> {
        set_verifier_handler(ctx, new_verifier)
    }

    pub fn pause<'info>(ctx: Context<Setting<'info>>) -> Result<()> {
        pause_handler(ctx)
    }

    pub fn resume<'info>(ctx: Context<Setting<'info>>) -> Result<()> {
        resume_handler(ctx)
    }

    pub fn set_global_generation_price<'info>(ctx: Context<Setting<'info>>, generation_price: u64) -> Result<()> {
        set_global_generation_price_handler(ctx, generation_price)
    }

    pub fn set_percentages<'info>(ctx: Context<Setting<'info>>, liquidity_percentage: u64, main_referral_percentage: u64, second_referral_percentage: u64) -> Result<()> {
        set_percentages_handler(ctx, liquidity_percentage, main_referral_percentage, second_referral_percentage)
    }
}
