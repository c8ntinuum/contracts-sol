// programs/num-token/src/instructions/initialize.rs
use crate::*;

pub fn set_percentages_handler<'info>(
    ctx: Context<Setting<'info>>,    //
    liquidity_percentage: u64,       //
    main_referral_percentage: u64,   //
    second_referral_percentage: u64, //
) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }

    // Validate percentages
    if PERCENTAGE_DIVIDER <= liquidity_percentage + main_referral_percentage + second_referral_percentage
        || liquidity_percentage == 0
        || main_referral_percentage == 0
        || second_referral_percentage == 0
    {
        return Err(CustomError::InvalidPercentages.into());
    }

    // Change percentages
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.liquidity_percentage = liquidity_percentage;
    config_account.main_referral_percentage = main_referral_percentage;
    config_account.second_referral_percentage = second_referral_percentage;

    msg!(
        "Percentages changed LP: {}, MR: {}, SR: {}",
        config_account.liquidity_percentage,
        config_account.main_referral_percentage,
        config_account.second_referral_percentage
    );

    Ok(())
}
