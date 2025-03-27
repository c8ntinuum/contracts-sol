//src/instructions/add_to_black_list.rs
use crate::*;

pub fn add_to_black_list_handler<'info>(ctx: Context<SettingWithSystemProgram<'info>>, address: Pubkey) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }

    // Check black listed address is not empty address
    if address == Pubkey::default() {
        return Err(CustomError::InvalidPubKey.into());
    }

    // Check if user is already blacklisted
    if ctx.accounts.config_account.black_list.contains(&address) {
        return Err(CustomError::UserIsAlreadyBlackListed.into());
    }

    // Check if configuration account resize is needed
    if ctx.accounts.config_account.black_list.len() != 0 && ctx.accounts.config_account.black_list.len() % BLACKLIST_LOT_SIZE == 0 {
        // Transfer some amount to config account to have money for rent during reallocation
        transfer_sol(
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.config_account.to_account_info(),
            25_000_000, //0.02436 SOL = 24,360,000 lamports to store extra 100 slots
        )?;
        let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
        config_account
            .to_account_info()
            .realloc(ctx.accounts.config_account.to_account_info().data_len() + 32 * BLACKLIST_LOT_SIZE, true)?;
    }

    // Add address to black list
    let config_account: &mut Account<'_, ConfigAccount> = &mut ctx.accounts.config_account;
    config_account.black_list.push(address);
    Ok(())
}
