use anchor_lang::{prelude::*, system_program};

use crate::state::Pool;

pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let referral_account = ctx.accounts.referral_account.as_ref();

    let referral_fee = amount * pool.referral_fee_percentage / 10000;

    if referral_account.is_some() && !pool.referral_lockdown {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: referral_account.unwrap().to_account_info(),
                },
            ),
            referral_fee,
        )?;
    }

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.fund_wallet.to_account_info(),
            },
        ),
        amount - referral_fee,
    )?;

    pool.buy(amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Buy<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK:
    #[account(
        mut,
        constraint = fund_wallet.key() == pool.fund_wallet
    )]
    pub fund_wallet: AccountInfo<'info>,

    /// CHECK: This account is optional
    pub referral_account: Option<AccountInfo<'info>>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
