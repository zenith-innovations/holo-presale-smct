use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Admin configuration already exists")]
    AdminConfigurationAlreadyExists,

    #[msg("Wrong admin wallet")]
    WrongAdminWallet,

    #[msg("Invalid fund wallet")]
    InvalidFundWallet,

    #[msg("Sold Out")]
    SoldOut,

    #[msg("Not Active")]
    NotActive,

    #[msg("Invalid Time")]
    InvalidTime,
}