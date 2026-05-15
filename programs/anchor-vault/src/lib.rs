pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6BWdnaDPj72DUNia6pwk1xzfndrF5bbVqfrJefRCe1rj");

#[program]
pub mod anchor_vault {
  use super::*;
  
  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    Initialize::handler(ctx)
  }
  
  pub fn deposit(ctx: Context<Deposit>, amount: u64, next_unlock_time: Option<i64>) -> Result<()> {
    Deposit::handler(ctx, amount, next_unlock_time)
  }
  
  pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    Withdraw::handler(ctx, amount)
  }
  
  pub fn close(ctx: Context<Close>) -> Result<()> {
    Close::handler(ctx)
  }
}
