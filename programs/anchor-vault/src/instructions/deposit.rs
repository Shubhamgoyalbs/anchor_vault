use anchor_lang::prelude::*;
use anchor_lang::system_program::{
  transfer,
  Transfer
};
use crate::VaultState;

#[derive(Accounts)]
pub struct Deposit<'info> {
  #[account[mut]]
  pub signer: Signer<'info>,
  
  #[account(
    seeds = [
      b"vault_state",
      signer.key().as_ref()
    ],
    bump = vault_state.state_bump
  )]
  pub vault_state: Account<'info, VaultState>,
  
  #[account(
    mut,
    seeds = [
      b"vault",
      signer.key().as_ref()
    ],
    bump = vault_state.vault_bump
  )]
  pub vault: SystemAccount<'info>,
  
  pub system_program: Program<'info, System>
}

impl<'info> Deposit<'info> {
  pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    transfer(
      CpiContext::new(
        ctx.accounts.system_program.key(),
        Transfer {
          from: ctx.accounts.signer.to_account_info(),
          to: ctx.accounts.vault.to_account_info()
        }
      ),
      amount
    )?;
    Ok(())
  }
}