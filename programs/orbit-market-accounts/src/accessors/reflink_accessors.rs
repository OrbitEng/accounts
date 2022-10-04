use anchor_lang::{prelude::*, AccountsClose};

use crate::{OrbitMarketAccount, OrbitReflink, MarketAccountErrors};

#[derive(Accounts)]
pub struct CreateReflink<'info>{

    #[account(
        init,
        space = 400,
        seeds = [
            b"orbit_reflink",
            market_account.key().as_ref()
        ],
        bump,

        payer = wallet
    )]
    pub reflink: Account<'info, OrbitReflink>,

    #[account(
        mut,
        constraint = market_account.owned_reflink == Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]),
        constraint = (Clock::get()?.unix_timestamp - market_account.account_created) > 604800,
        constraint = market_account.transactions > 3,
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump
    )]
    pub market_account:Box<Account<'info, OrbitMarketAccount>>,

    
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
    ctx.accounts.market_account.owned_reflink = ctx.accounts.reflink.key();
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteReflink<'info>{
    #[account(
        mut
    )]
    pub reflink: Account<'info, OrbitReflink>,

    #[account(
        mut,
        address = reflink.owner,
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump
    )]
    pub market_account:Box<Account<'info, OrbitMarketAccount>>,

    
    #[account(
        mut,
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>
}

pub fn delete_reflink_handler(ctx: Context<DeleteReflink>) -> Result<()>{
    if ctx.remaining_accounts.len() != ctx.accounts.reflink.users.len(){
        return err!(MarketAccountErrors::MismatchedUsersForReflink);
    };

    for user in ctx.accounts.reflink.users.iter().enumerate(){
        if !((user.1 == &ctx.remaining_accounts[user.0].key()) && (ctx.remaining_accounts[user.0].is_writable)){
            return err!(MarketAccountErrors::MismatchedUsersForReflink);
        };

        let mut user_acc = Account::<OrbitMarketAccount>::try_from(&ctx.remaining_accounts[user.0]).expect(format!("could not deserialize user account {:?}", user.0).as_str());
        user_acc.reflink = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    }
    
    ctx.accounts.reflink.close(ctx.accounts.wallet.to_account_info()).expect("could not close reflink account");
    ctx.accounts.market_account.owned_reflink = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

    Ok(())
}