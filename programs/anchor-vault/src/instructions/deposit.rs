use anchor_lang::prelude::*;
use anchor_lang::{
  system_program::{
    transfer,
    Transfer,
  },
  solana_program::{
    clock::Clock
  }
};
use crate::state::VaultState;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Deposit<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
  
  #[account(
    mut,
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
  //next_unlock_time is required when previous lock get unlocked, now you want to set new one
  pub fn handler(ctx: Context<Deposit>, amount: u64, next_unlock_time: Option<i64>) -> Result<()> {
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
    
    match next_unlock_time {
      None => {}
      Some(time_stamp) => {
        let current_time_stamp = Clock::get()?.unix_timestamp;
        match ctx.accounts.vault_state.time_stamp {
          None => {
            require!(time_stamp > current_time_stamp, ErrorCode::InvalidTimeStamp);
            ctx.accounts.vault_state.time_stamp = Some(time_stamp);
          }
          Some(val) => {
            if val <= current_time_stamp {
              require!(time_stamp > current_time_stamp, ErrorCode::InvalidTimeStamp);
              ctx.accounts.vault_state.time_stamp = Some(time_stamp);
            }
          }
        }
      }
    }
    
    Ok(())
  }
}