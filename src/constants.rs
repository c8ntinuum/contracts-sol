// programs/num-token/src/constants.rs
use solana_program::{pubkey, pubkey::Pubkey};

pub const CONFIG_SEED: &'static str = "config";
pub const VAULT_ACCOUNT_SEED: &'static str = "vault";

pub const PERCENTAGE_DIVIDER: u64 = 1_000; //100%
pub const PERCENTAGE_TO_LIQUIDITY: u64 = 200; //20%
pub const PERCENTAGE_MAIN_REFERRAL: u64 = 20; //2%
pub const PERCENTAGE_SECOND_REFERRAL: u64 = 80; //8%
pub const GENERATION_MIN_AMOUNT: u64 = 5_000_000; // 0.005 SOL
pub const MIN_FLOOR_GENERATION_PRICE_USD: u64 = 22_400; // 0.0224 * 10^6 -> 0.0224$ - USDT is 6 decimals on Solana
pub const SERVER_SIGNER_PUB: Pubkey = pubkey!("31cVhGCx453d4xWrXmUTK1pE6CzxdD1QQH9cMwbbwyU6");
pub const MAX_BLACKLIST_SIZE: usize = 300_000; // max amount of black listed addresses to fit, actually is 327_676
pub const BLACKLIST_LOT_SIZE: usize = 100; // we reallocate data for the account every 100 black list elements

pub const NO_REFERRAL: Pubkey = pubkey!("YMN9Qj5jPNp7j14VPcML1B6xGgcPWVZUGLFU3Mnyfaf");
pub const WSOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const C8NT_MINT: Pubkey = pubkey!("GnbVA1SgBSn1uPqxzyjZTHYAs9FNwb4ykUgRZY2S6T4e");
pub const USDT_MINT: Pubkey = pubkey!("2d91XomsDYhRRkiaWhr3mds8cN1Quf66HgPaRw6t718n");
pub const SWAP_PROGRAM_ID: Pubkey = pubkey!("77GvZUyTXUzUuaUu3M6AducM56RkZpyoHP4NHna9CLvo");

pub const C8NT_WSOL_POOL_ID: Pubkey = pubkey!("32z19fRXLcBKeKVLeHVrjD6e1nSmQCEfEZyCFdGGc8uw");
pub const C8NT_WSOL_POOL_AMM_CONFIG: Pubkey = pubkey!("zHBt9QYknK9tyZEJTkmUYjh2zoJ8pEk5t5dVU9LPjTG");
pub const C8NT_WSOL_POOL_LP_MINT: Pubkey = pubkey!("4oyi61Jk7F55bmpc1jadrEGCmKvWX76pvxs3QvQiHHjn");
pub const C8NT_WSOL_POOL_VAULT_WSOL: Pubkey = pubkey!("AxJ1fBmyzubxemtMDAVAX1CaR4u8HFBkmt2XZLuLLyBH");
pub const C8NT_WSOL_POOL_VAULT_C8NT: Pubkey = pubkey!("FiM9Q11JBDHF76krFK1k1kiPuinXEhzCM6VqSfGrYUBE");
pub const C8NT_WSOL_POOL_OBSERVATION_STATE: Pubkey = pubkey!("DSFZjJVrQBdn6KB2Lo97HotLRa37p3RRuF2M9znXHW8h");
pub const C8NT_WSOL_POOL_AUTHORITY: Pubkey = pubkey!("9SZDVj8hThyMfaquw1hNgKFzu58mBm2GAjSCMyu9BNo2");

pub const USDT_WSOL_POOL_ID: Pubkey = pubkey!("2RDJwPFLzGWoSQsyscU5U4reTnFWh3JibwcEcoDKQbX8");
pub const USDT_WSOL_POOL_VAULT_WSOL: Pubkey = pubkey!("5MpXknDjNwikSyGjNPhn6VHWH9GgRHLQnLnAALeSc4N7");
pub const USDT_WSOL_POOL_VAULT_USDT: Pubkey = pubkey!("6ow5mMSwRBdezsW7ZFQ7hCRm6P49Ku62UZo5HbHWU2uf");
