//src/instructions/set_verifier.rs
use crate::*;

pub fn set_verifier_handler<'info>(ctx: Context<Setting<'info>>, new_verifier: Pubkey) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }
    // Check if new verifier is not empty address
    if new_verifier == Pubkey::default() {
        return Err(CustomError::InvalidPubKey.into());
    }
    // Check if new verifier is different
    if new_verifier == ctx.accounts.config_account.verifier {
        return Err(CustomError::SameVerifier.into());
    }
    // Add new verifier
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.verifier = new_verifier;
    Ok(())
}
