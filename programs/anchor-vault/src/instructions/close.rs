use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::VaultState;

#[derive(Accounts)]
pub struct Close<'info> {
  #[account[mut]]
  pub signer: Signer<'info>,
  
  #[account(
    mut,
    seeds = [
      b"vault_state",
      signer.key().as_ref()
    ],
    close = signer,    bump = vault_state.state_bump
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

impl<'info> Close<'info> {
  pub fn handler(ctx: Context<Close>) -> Result<()> {
    let seeds = &[
      b"vault",
      ctx.accounts.signer.key.as_ref(),
      &[
        ctx.accounts.vault_state.vault_bump
      ]
    ];
    
    let signer_seeds = &[
      &seeds[..]
    ];
    transfer(
      CpiContext::new_with_signer(
        ctx.accounts.system_program.key(),
        Transfer {
          from: ctx.accounts.vault.to_account_info(),
          to: ctx.accounts.signer.to_account_info()
        },
        signer_seeds
      ),
      ctx.accounts.vault.lamports()
    )?;
    Ok(())
  }
}