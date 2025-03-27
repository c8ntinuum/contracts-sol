//src/instructions/set_withdrawal_address.rs
use crate::*;

pub fn set_withdrawal_address_handler<'info>(ctx: Context<Setting<'info>>, new_withdrawal_address: Pubkey) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }
    // Check if new withdrawal_address is not empty address
    if new_withdrawal_address == Pubkey::default() {
        return Err(CustomError::InvalidPubKey.into());
    }
    // Check if new withdrawal_address is different
    if new_withdrawal_address == ctx.accounts.config_account.withdrawal_address {
        return Err(CustomError::SameWithdrawalAddress.into());
    }
    // Set new withdrawal_address
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.withdrawal_address = new_withdrawal_address;
    Ok(())
}
