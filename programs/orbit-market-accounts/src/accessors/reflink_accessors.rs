use anchor_lang::prelude::*;

use crate::{OrbitMarketAccount, OrbitReflink};

#[derive(Accounts)]
pub struct CreateReflink<'info>{

    #[account(
        init,
        space = 64,
        seeds = [
            b"reflink",
            market_account.key().as_ref()
        ],
        bump,

        payer = payer
    )]
    pub reflink: Account<'info, OrbitReflink>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = market_account.master_pubkey
    )]
    pub market_auth: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn init_reflink_handler(ctx: Context<CreateReflink>) -> Result<()>{
    ctx.accounts.reflink.uses = 0;
    Ok(())
}