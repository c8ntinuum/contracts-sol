//src/constants.rs
use solana_program::{pubkey, pubkey::Pubkey};

pub const CONFIG_SEED: &'static str = "config";
pub const VAULT_ACCOUNT_SEED: &'static str = "vault";

pub const PERCENTAGE_DIVIDER: u64 = 1_000; //100%
pub const PERCENTAGE_TO_LIQUIDITY: u64 = 400; //40%
pub const PERCENTAGE_MAIN_REFERRAL: u64 = 20; //2%
pub const PERCENTAGE_SECOND_REFERRAL: u64 = 80; //8%
pub const GENERATION_MIN_AMOUNT: u64 = 100_000_000; // 0.1 SOL
pub const MIN_FLOOR_GENERATION_PRICE_USD: u64 = 22_400; // 0.0224 * 10^6 -> 0.0224$ - USDT is 6 decimals on Solana
pub const MAX_BLACKLIST_SIZE: usize = 300_000; // max amount of black listed addresses to fit, actually is 327_676
pub const BLACKLIST_LOT_SIZE: usize = 100; // we reallocate data for the account every 100 black list elements
pub const WSOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const NO_REFERRAL: Pubkey = pubkey!("YMN9Qj5jPNp7j14VPcML1B6xGgcPWVZUGLFU3Mnyfaf");

pub const USDT_MINT: Pubkey = pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
pub const SWAP_PROGRAM_ID: Pubkey = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
pub const USDT_WSOL_POOL_ID: Pubkey = pubkey!("3nMFwZXwY1s1M5s8vYAHqd4wGs4iSxXE4LRoUMMYqEgF");

pub const SERVER_SIGNER_PUB: Pubkey = pubkey!("918t98ZYAfeXW9WSA7KD4xc7kjAHwmzELxDR2vLK7Ba");
pub const C8NT_MINT: Pubkey = pubkey!("C8qKVjKNf6JH2yrVVf3aFyny6ZCbHCJUBXTQAT2Em888");
pub const C8NT_WSOL_POOL_ID: Pubkey = pubkey!("ASkLF8sxANUgR57hd8YozWrqR6mwprwRC54v9tnstTwT");
pub const C8NT_WSOL_POOL_AMM_CONFIG: Pubkey = pubkey!("D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2");
pub const C8NT_WSOL_POOL_LP_MINT: Pubkey = pubkey!("7piwoZpNrRmP7qWmBjXrmaDQFtLZUGmcGUTSvbTLBE38");
pub const C8NT_WSOL_POOL_VAULT_WSOL: Pubkey = pubkey!("3rrqHDgBgmgwr7HZC6RcgSK2UKJuFKeEzDkS5coV1Ytz");
pub const C8NT_WSOL_POOL_VAULT_C8NT: Pubkey = pubkey!("8mSgqP6qx8jiABDkkSkzY14dsf7iSLzZWhjLD4xwEv9r");
pub const C8NT_WSOL_POOL_OBSERVATION_STATE: Pubkey = pubkey!("3JoumiJ8dEbBzVuAcCz2LWDLiHjZ5GrucHtMzviSz7xF");
pub const C8NT_WSOL_POOL_AUTHORITY: Pubkey = pubkey!("GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL");
