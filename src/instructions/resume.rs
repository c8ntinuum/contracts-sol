// programs/num-token/src/instructions/initialize.rs
use crate::*;

pub fn resume_handler<'info>(ctx: Context<Setting<'info>>) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }

    // Pause contract
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.is_paused = false;
    msg!("Contract resumed, is_pause set to {}", config_account.is_paused);

    Ok(())
}
