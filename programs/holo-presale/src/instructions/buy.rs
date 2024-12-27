use anchor_lang::{prelude::*, system_program};

use crate::state::Pool;

pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let referral_fee = amount * pool.referral_fee_percentage / 10000;

    if !pool.referral_lockdown {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.referral_account.to_account_info(),
                },
            ),
            referral_fee,
        )?;
    } else {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.fund_wallet.to_account_info(),
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

    if !pool.referral_lockdown {
        emit!(BuyEvent {
            amount,
            referral_fee,
            referral_account: ctx.accounts.referral_account.key(),
        });
    } else {
        emit!(BuyEvent {
            amount,
            referral_fee: 0,
            referral_account: ctx.accounts.referral_account.key(),
        });
    }

    let user_purchase = &mut ctx.accounts.user_purchase;
    user_purchase.total_purchased += amount;

    pool.buy(amount)?;

    Ok(())
}

#[account]
pub struct UserPurchase {
    pub total_purchased: u64,
}

impl UserPurchase {
    pub const LEN: usize = 8 + 8; // 8 bytes for discriminator, 8 for u64
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Buy<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This account is optional
    #[account(
        mut,
        constraint = fund_wallet.key() == pool.fund_wallet
    )]
    pub fund_wallet: AccountInfo<'info>,

    /// CHECK: This account is default address if no referral
    #[account(mut)]
    pub referral_account: AccountInfo<'info>,

    #[account(init_if_needed, seeds = [b"user_purchase", user.key().as_ref(), pool.key().as_ref()],payer = user, space = UserPurchase::LEN, bump)]
    pub user_purchase: Box<Account<'info, UserPurchase>>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct BuyEvent {
    pub amount: u64,
    pub referral_fee: u64,
    pub referral_account: Pubkey,
}
