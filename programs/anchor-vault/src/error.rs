use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Funds are not yet released")]
    Locked,
}
