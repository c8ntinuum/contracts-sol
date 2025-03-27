//src/instructions/resume.rs
use crate::*;

pub fn resume_handler<'info>(ctx: Context<Setting<'info>>) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }
    // Check if contract pause status is already false
    if !ctx.accounts.config_account.is_paused {
        return Err(CustomError::SamePause.into());
    }
    // Resume contract
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.is_paused = false;
    Ok(())
}
