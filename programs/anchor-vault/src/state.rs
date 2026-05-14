use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState {
  pub vault_bump: u8,
  pub state_bump: u8,
  pub time_stamp: Option<i64>, // unix time stamp (Clock) after which funds in vault are released,
  // can be reset after this get unlocked, by depositing sol in this
}