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

    emit!(PoolInitialized {
        is_active,
        referral_fee_percentage,
        referral_lockdown,
        admin_wallet,
        fund_wallet,
        sale_amount,
        start_time,
        end_time,
    });

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

    emit!(PoolUpdated {
        is_active,
        referral_fee_percentage,
        referral_lockdown,
        admin_wallet,
        fund_wallet,
        sale_amount,
        start_time,
        end_time,
    });

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
    #[account(
        mut,
        constraint = admin.key() == pool.fund_wallet @ CustomError::WrongAdminWallet
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub pool: Box<Account<'info, Pool>>,
}


#[event]
pub struct PoolUpdated {
    pub is_active: bool,
    pub referral_fee_percentage: u64,
    pub referral_lockdown: bool,
    pub admin_wallet: Pubkey,
    pub fund_wallet: Pubkey,
    pub sale_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[event]
pub struct PoolInitialized {
    pub is_active: bool,
    pub referral_fee_percentage: u64,
    pub referral_lockdown: bool,
    pub admin_wallet: Pubkey,
    pub fund_wallet: Pubkey,
    pub sale_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
}
