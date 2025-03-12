// programs/num-token/src/instructions/initialize.rs
use crate::*;

pub fn withdraw_handler<'info>(ctx: Context<Withdraw<'info>>) -> Result<()> {
    // Validate admin
    if ctx.accounts.signer.key() != ctx.accounts.config_account.admin {
        return Err(CustomError::Unauthorized.into());
    }

    // Check vault amount
    require!(ctx.accounts.vault.to_account_info().lamports() > 5_000_000, CustomError::InsufficientBalance);

    // Init vault pda seeds user to sign vault transfers
    let seeds: &[&[u8]; 2] = &[VAULT_ACCOUNT_SEED.as_bytes(), &[ctx.bumps.vault]];
    let vault_seeds: &[&[&[u8]]] = &[seeds];

    // Calculate withdrawal amount, leave 5M lamport for rent
    let amount: u64 = ctx.accounts.vault.to_account_info().lamports() - 5_000_000;

    // Withdraw lamport from vault to mentioned receiver
    transfer_sol_from_pda(
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.receiver.to_account_info(),
        amount,
        vault_seeds,
    )?;
    msg!("Successfully withdrawn {}", amount);
    Ok(())
}
