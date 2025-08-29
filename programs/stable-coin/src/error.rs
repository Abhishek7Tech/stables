use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,

    #[msg("Below minimum health factor")]
    BelowMinimumHealthFactor,

    #[msg("Can't liquidate a healthy account")]
    AboveMinimumHealthFactor

}