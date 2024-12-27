use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("AawKke79hDe8sa3Luheb7ckktrqAviH8n7M3Ugi5tQBy");

#[program]
pub mod holo_presale {

    use super::*;

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        instructions::buy::buy(ctx, amount)
    }

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        is_active: bool,
        referral_fee_percentage: u64,
        referral_lockdown: bool,
        admin_wallet: Pubkey,
        fund_wallet: Pubkey,
        sale_amount: u64,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        let _ = instructions::admin::initialize_pool(
            ctx,
            is_active,
            referral_fee_percentage,
            referral_lockdown,
            admin_wallet,
            fund_wallet,
            sale_amount,
            start_time,
            end_time,
        );
        Ok(())
    }

    pub fn update_pool(
        ctx: Context<UpdatePool>,
        is_active: bool,
        referral_fee_percentage: u64,
        referral_lockdown: bool,
        admin_wallet: Pubkey,
        fund_wallet: Pubkey,
        sale_amount: u64,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        let _ = instructions::admin::update_pool(
            ctx,
            is_active,
            referral_fee_percentage,
            referral_lockdown,
            admin_wallet,
            fund_wallet,
            sale_amount,
            start_time,
            end_time,
        );
        Ok(())
    }
}
