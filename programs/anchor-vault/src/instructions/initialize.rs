use anchor_lang::prelude::*;
use crate::state::VaultState;

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account[mut]]
  pub signer: Signer<'info>,
  
  #[account(
    init,
    payer = signer,
    space = 8 + VaultState::INIT_SPACE,
    seeds = [
      b"vault_state",
      signer.key().as_ref()
    ],
    bump
  )]
  pub vault_state: Account<'info, VaultState>,
  
  #[account(
    seeds = [
      b"vault",
      signer.key().as_ref()
    ],
    bump
  )]
  pub vault: SystemAccount<'info>,
  
  pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
  pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.vault_state.state_bump = ctx.bumps.vault_state;
    ctx.accounts.vault_state.vault_bump = ctx.bumps.vault;
    Ok(())
  }
}