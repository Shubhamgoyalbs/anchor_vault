use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Funds are not yet released")]
    Locked,
    #[msg("Time stamp must be of future as compared to current time")]
    InvalidTimeStamp
}
