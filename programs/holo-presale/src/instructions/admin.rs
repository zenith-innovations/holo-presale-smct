use crate::{errors::CustomError, state::*};
use anchor_lang::prelude::*;

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
    let pool = &mut ctx.accounts.pool;

    pool.set_inner(Pool::new(
        is_active,
        referral_fee_percentage,
        referral_lockdown,
        admin_wallet,
        fund_wallet,
        sale_amount,
        start_time,
        end_time,
    ));

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
    let pool = &mut ctx.accounts.pool;

    if ctx.accounts.admin.key() != pool.admin_wallet {
        return Err(CustomError::WrongAdminWallet.into());
    }

    let _ = pool.update(
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

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        space = Pool::SIZE,
        payer = admin,
        seeds = [Pool::SEED.as_bytes()],
        bump,
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub pool: Box<Account<'info, Pool>>,
}
