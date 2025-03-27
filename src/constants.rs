//src/constants.rs
use solana_program::{pubkey, pubkey::Pubkey};

pub const CONFIG_SEED: &'static str = "config";
pub const VAULT_ACCOUNT_SEED: &'static str = "vault";

pub const PERCENTAGE_DIVIDER: u64 = 1_000; //100%
pub const PERCENTAGE_TO_LIQUIDITY: u64 = 200; //20%
pub const PERCENTAGE_MAIN_REFERRAL: u64 = 20; //2%
pub const PERCENTAGE_SECOND_REFERRAL: u64 = 80; //8%
pub const GENERATION_MIN_AMOUNT: u64 = 100_000_000; // 0.1 SOL
pub const MIN_FLOOR_GENERATION_PRICE_USD: u64 = 22_400; // 0.0224 * 10^6 -> 0.0224$ - USDT is 6 decimals on Solana
pub const MAX_BLACKLIST_SIZE: usize = 300_000; // max amount of black listed addresses to fit, actually is 327_676
pub const BLACKLIST_LOT_SIZE: usize = 100; // we reallocate data for the account every 100 black list elements
pub const WSOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const NO_REFERRAL: Pubkey = pubkey!("YMN9Qj5jPNp7j14VPcML1B6xGgcPWVZUGLFU3Mnyfaf");

// LOCAL ENVIRONMENT
// pub const USDT_MINT: Pubkey = pubkey!("2d91XomsDYhRRkiaWhr3mds8cN1Quf66HgPaRw6t718n");
// pub const SWAP_PROGRAM_ID: Pubkey = pubkey!("77GvZUyTXUzUuaUu3M6AducM56RkZpyoHP4NHna9CLvo");
// pub const USDT_WSOL_POOL_ID: Pubkey = pubkey!("2RDJwPFLzGWoSQsyscU5U4reTnFWh3JibwcEcoDKQbX8");

// LOCAL ENVIRONMENT
// pub const SERVER_SIGNER_PUB: Pubkey = pubkey!("31cVhGCx453d4xWrXmUTK1pE6CzxdD1QQH9cMwbbwyU6");
// pub const C8NT_MINT: Pubkey = pubkey!("GnbVA1SgBSn1uPqxzyjZTHYAs9FNwb4ykUgRZY2S6T4e");
// pub const C8NT_WSOL_POOL_ID: Pubkey = pubkey!("32z19fRXLcBKeKVLeHVrjD6e1nSmQCEfEZyCFdGGc8uw");
// pub const C8NT_WSOL_POOL_AMM_CONFIG: Pubkey = pubkey!("zHBt9QYknK9tyZEJTkmUYjh2zoJ8pEk5t5dVU9LPjTG");
// pub const C8NT_WSOL_POOL_LP_MINT: Pubkey = pubkey!("4oyi61Jk7F55bmpc1jadrEGCmKvWX76pvxs3QvQiHHjn");
// pub const C8NT_WSOL_POOL_VAULT_WSOL: Pubkey = pubkey!("AxJ1fBmyzubxemtMDAVAX1CaR4u8HFBkmt2XZLuLLyBH");
// pub const C8NT_WSOL_POOL_VAULT_C8NT: Pubkey = pubkey!("FiM9Q11JBDHF76krFK1k1kiPuinXEhzCM6VqSfGrYUBE");
// pub const C8NT_WSOL_POOL_OBSERVATION_STATE: Pubkey = pubkey!("DSFZjJVrQBdn6KB2Lo97HotLRa37p3RRuF2M9znXHW8h");
// pub const C8NT_WSOL_POOL_AUTHORITY: Pubkey = pubkey!("9SZDVj8hThyMfaquw1hNgKFzu58mBm2GAjSCMyu9BNo2");

// LIVE ENVIRONMENT
pub const USDT_MINT: Pubkey = pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
pub const SWAP_PROGRAM_ID: Pubkey = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
pub const USDT_WSOL_POOL_ID: Pubkey = pubkey!("3nMFwZXwY1s1M5s8vYAHqd4wGs4iSxXE4LRoUMMYqEgF");

// LIVE ENVIRONMENT
pub const SERVER_SIGNER_PUB: Pubkey = pubkey!("31cVhGCx453d4xWrXmUTK1pE6CzxdD1QQH9cMwbbwyU6");
pub const C8NT_MINT: Pubkey = pubkey!("FW4UAqJZzrKA78ftyQoQZgjLeRCWzG9osMNU9EeqZWut");
pub const C8NT_WSOL_POOL_ID: Pubkey = pubkey!("AvAYTBVKbnKeAcijxDuqoTLiWH42Pmr8UwkdSHykPQ9h");
pub const C8NT_WSOL_POOL_AMM_CONFIG: Pubkey = pubkey!("D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2");
pub const C8NT_WSOL_POOL_LP_MINT: Pubkey = pubkey!("HDKsR7Wbr6PT7CH88ZfqH8VQpyb7JmuzQyHjU49BADK");
pub const C8NT_WSOL_POOL_VAULT_WSOL: Pubkey = pubkey!("5ZahEDohqF2jwDSQzoUk8yqmJiDEhodDafdG1RMjna93");
pub const C8NT_WSOL_POOL_VAULT_C8NT: Pubkey = pubkey!("E1ivc6N5u2AC3LSKwCMWJ7YFBPEb1xNys2zbjGXYC59e");
pub const C8NT_WSOL_POOL_OBSERVATION_STATE: Pubkey = pubkey!("5S9mUuVgMvKXJPXhNgWxHjHaVhero9t4ihCAThvs37xw");
pub const C8NT_WSOL_POOL_AUTHORITY: Pubkey = pubkey!("GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL");
