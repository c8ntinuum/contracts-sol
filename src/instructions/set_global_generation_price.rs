//src/instructions/set_global_generation_price.rs
use crate::*;

pub fn set_global_generation_price_handler<'info>(ctx: Context<Setting<'info>>, generation_price: u64) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }
    // Validate generation price
    if generation_price < MIN_FLOOR_GENERATION_PRICE_USD {
        return Err(CustomError::GenerationPriceLessThanMin.into());
    }
    // Change global generation price
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.global_generation_price_usd = generation_price;
    Ok(())
}
