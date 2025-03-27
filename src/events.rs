//src/events.rs
use anchor_lang::prelude::*;

#[event]
pub struct GenerationCreated {
    pub signer: Pubkey,
    pub sol_amount: u64,
    pub sol_amount_in_usdt: u64,
    pub ctnm_usd_price_oracle: u64,
    pub ctnm_usd_price_pool: u64,
    pub slippage: u16,
    pub referral1: Pubkey,
    pub referral2: Pubkey,
    pub signature: String,
}
