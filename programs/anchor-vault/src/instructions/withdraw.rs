use anchor_lang::prelude::*;
use anchor_lang::{
  system_program::{
    transfer,
    Transfer
  },
  solana_program::{
    clock::Clock
  }
};
use crate::{
  state::VaultState,
  error::ErrorCode
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
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

impl<'info> Withdraw<'info> {
  pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    match ctx.accounts.vault_state.time_stamp {
      None => {}
      Some(time_stamp) => {
        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(
          time_stamp < current_timestamp,
          ErrorCode::Locked
        );
      }
    }
    
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
      amount
    )?;
    Ok(())
  }
}