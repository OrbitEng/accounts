use anchor_lang::prelude::*;

use crate::{OrbitMarketAccount, OrbitReflink};

#[derive(Accounts)]
pub struct CreateReflink<'info>{

    #[account(
        init,
        space = 64,
        seeds = [
            b"orbit_reflink",
            market_account.key().as_ref()
        ],
        bump,

        payer = wallet
    )]
    pub reflink: Account<'info, OrbitReflink>,

    #[account(
        constraint = (Clock::get()?.unix_timestamp - market_account.account_created) > 604800,
        constraint = market_account.transactions > 3,
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump
    )]
    pub market_account: Account<'info, OrbitMarketAccount>,

    
    #[account(
        mut,
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn init_reflink_handler(ctx: Context<CreateReflink>) -> Result<()>{
    ctx.accounts.reflink.uses = 0;
    ctx.accounts.reflink.owner = ctx.accounts.market_account.key();
    Ok(())
}