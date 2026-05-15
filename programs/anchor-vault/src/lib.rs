pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EE74Pp8MwWrUnXMpK8j1tW115uSMH9RR1MS8wjvVEG7V");

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
