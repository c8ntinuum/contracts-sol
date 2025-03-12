// programs/num-token/src/instructions/initialize.rs
use crate::*;

pub fn set_admin_handler<'info>(ctx: Context<Setting<'info>>, new_admin: Pubkey) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }

    // Check if new admin is not empty address, on curve check is NOT required because new admin might be a multi signature account
    if new_admin == Pubkey::default() {
        return Err(CustomError::InvalidPubKey.into());
    }

    // Check if new admin is different
    if new_admin == ctx.accounts.config_account.admin.key() {
        return Err(CustomError::SameAdmin.into());
    }

    // Add new admin
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.admin = new_admin;
    msg!("New admin set to {}", config_account.admin);

    Ok(())
}
