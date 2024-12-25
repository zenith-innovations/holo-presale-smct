use anchor_lang::prelude::*;

use crate::errors::CustomError;

#[account]
pub struct Pool {
    pub is_active: bool, // if true, pool is active
    pub referral_fee_percentage: u64, // 100 = 1%
    pub referral_lockdown: bool,      // if true, referral fee is transfer to admin wallet
    pub admin_wallet: Pubkey,         // admin wallet
    pub fund_wallet: Pubkey, // fund wallet
    pub sale_amount: u64,    // sale amount in SOL
    pub sold_amount: u64,    // sold amount in SOL
    pub start_time: i64,     // start time in UTC seconds
    pub end_time: i64,       // end time in UTC seconds
}

impl Pool {
    pub const SEED: &'static str = "AdminConfiguration";
    pub const SIZE: usize = 8 + 1 + 8 + 1 + 32 + 32 + 8 + 8 + 8 + 8;

    pub fn new(
        is_active: bool,
        referral_fee_percentage: u64,
        referral_lockdown: bool,
        admin_wallet: Pubkey,
        fund_wallet: Pubkey,
        sale_amount: u64,
        start_time: i64,
        end_time: i64,
    ) -> Self {
        Self {
            is_active,
            referral_fee_percentage,
            referral_lockdown,
            admin_wallet,
            fund_wallet,
            sale_amount,
            sold_amount: 0,
            start_time,
            end_time,
        }
    }

    pub fn update(
        &mut self,
        is_active: bool,
        referral_fee_percentage: u64,
        referral_lockdown: bool,
        admin_wallet: Pubkey,
        fund_wallet: Pubkey,
        sale_amount: u64,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        self.is_active = is_active;
        self.referral_fee_percentage = referral_fee_percentage;
        self.referral_lockdown = referral_lockdown;
        self.admin_wallet = admin_wallet;
        self.fund_wallet = fund_wallet;
        self.sale_amount = sale_amount;
        self.start_time = start_time;
        self.end_time = end_time;
        Ok(())
    }

    pub fn get_admin_configuration(&self) -> Result<Pool> {
        Ok(self.clone())
    }

    pub fn validate_admin(&self, signer: &Pubkey) -> Result<()> {
        if self.admin_wallet != *signer {
            return Err(CustomError::WrongAdminWallet.into());
        }
        Ok(())
    }

    pub fn buy(&mut self, amount: u64) -> Result<()> {
        let clock = Clock::get()?;

        if self.sold_amount + amount > self.sale_amount {
            return Err(CustomError::SoldOut.into());
        }

        if !self.is_active {
            return Err(CustomError::NotActive.into());
        }

        if clock.unix_timestamp < self.start_time || clock.unix_timestamp > self.end_time {
            return Err(CustomError::InvalidTime.into());
        }

        self.sold_amount += amount;
        Ok(())
    }
}
