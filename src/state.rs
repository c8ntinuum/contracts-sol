// programs/num-token/src/state.rs
use crate::BLACKLIST_LOT_SIZE;
use anchor_lang::prelude::*;

#[account]
pub struct ConfigAccount {
    pub admin: Pubkey,                    // a.k.a. syncer - the pub key entitled to configure the program
    pub verifier: Pubkey,                 // pub key associated with signature sent by server, must match the signed payload
    pub liquidity_percentage: u64,        // an indicator of how much from a generation deposit is to gow in swap and add liquidity
    pub main_referral_percentage: u64,    // an indicator of how much , what percentage the main referral will receive from a deposit generation
    pub second_referral_percentage: u64,  // an indicator of how much , what percentage the second referral will receive from a deposit generation
    pub global_generation_price_usd: u64, //
    pub is_paused: bool,                  // an indicator to show if program is operable or not
    pub black_list: Vec<Pubkey>,          // Dynamically allocated, must be resized correctly
}

impl ConfigAccount {
    // 105 = 32(admin) + 32(verifier) + 8(liquidity_percentage) + 8(main_referral_percentage) + 8(second_referral_percentage) + 8(global_generation_price_usd) +1 (is_paused)
    pub const ACCOUNT_SIZE: usize = 8 + 105 + (BLACKLIST_LOT_SIZE * 32); // start with initial 100 black list elements
}

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct PoolState {
    /// Which config the pool belongs
    pub amm_config: Pubkey,
    /// pool creator
    pub pool_creator: Pubkey,
    /// Token A
    pub token_0_vault: Pubkey,
    /// Token B
    pub token_1_vault: Pubkey,

    /// Pool tokens are issued when A or B tokens are deposited.
    /// Pool tokens can be withdrawn back to the original A or B token.
    pub lp_mint: Pubkey,
    /// Mint information for token A
    pub token_0_mint: Pubkey,
    /// Mint information for token B
    pub token_1_mint: Pubkey,

    /// token_0 program
    pub token_0_program: Pubkey,
    /// token_1 program
    pub token_1_program: Pubkey,

    /// observation account to store oracle data
    pub observation_key: Pubkey,

    pub auth_bump: u8,
    /// Bitwise representation of the state of the pool
    /// bit0, 1: disable deposit(vaule is 1), 0: normal
    /// bit1, 1: disable withdraw(vaule is 2), 0: normal
    /// bit2, 1: disable swap(vaule is 4), 0: normal
    pub status: u8,

    pub lp_mint_decimals: u8,
    /// mint0 and mint1 decimals
    pub mint_0_decimals: u8,
    pub mint_1_decimals: u8,

    /// lp mint supply
    pub lp_supply: u64,
    /// The amounts of token_0 and token_1 that are owed to the liquidity provider.
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,

    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,

    /// The timestamp allowed for swap in the pool.
    pub open_time: u64,
    /// padding for future updates
    pub padding: [u64; 32],
}
impl PoolState {
    pub const LEN: usize = 8 + 10 * 32 + 1 * 5 + 8 * 6 + 8 * 32;
}
