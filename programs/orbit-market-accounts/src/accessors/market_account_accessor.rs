use anchor_lang::{prelude::*, system_program};
use crate::{
    structs::market_account::OrbitMarketAccount,
    errors::MarketAccountErrors
};

#[derive(Accounts)]
pub struct CreateMarketAccount<'info>{
    #[account(
        init,
        space = 250,
        payer = payer
    )]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub master_auth: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_account_handler(ctx: Context<CreateMarketAccount>, metadata_link: [u8; 64], payer_as_wallet: bool) -> Result<()>{
    if ctx.accounts.master_auth.owner.key() != system_program::ID{
        return err!(MarketAccountErrors::InvalidMasterPubkey)
    };
    let clock = Clock::get().expect("Could not get CLOCK SYSVAR");

    if payer_as_wallet{
        ctx.accounts.market_account.wallet = ctx.accounts.payer.key();
    }

    ctx.accounts.market_account.master_pubkey = ctx.accounts.master_auth.key();
    ctx.accounts.market_account.account_created = clock.unix_timestamp;
    ctx.accounts.market_account.metadata = metadata_link;

    // 人之初，性本善。性相近，习相远
    ctx.accounts.market_account.reputation = [0; 5];
    ctx.accounts.market_account.transactions = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct SetWallet<'info>{
    #[account(mut)]
    pub market_account: Account<'info, OrbitMarketAccount>,

    /// CHECK: New wallet must sign. This is for the safety of the users
    pub new_wallet: Signer<'info>,

    #[account(
        address = market_account.master_pubkey
    )]
    pub change_authority: Signer<'info>
}

pub fn set_wallet_handler(ctx: Context<SetWallet>) -> Result<()>{
    ctx.accounts.market_account.wallet = ctx.accounts.new_wallet.key();
    Ok(())
}

#[derive(Accounts)]
pub struct IncrementTransactions<'info>{
    #[account(mut)]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        constraint = 
        (invoker.key() == Pubkey::new(orbit_addresses::PHYSICAL_SIGNER)) ||
        (invoker.key() == Pubkey::new(orbit_addresses::DIGITAL_SIGNER)) ||
        (invoker.key() == market_account.master_pubkey)
    )]
    pub invoker: Signer<'info>,
}

pub fn post_tx_handler(ctx: Context<IncrementTransactions>) -> Result<()>{
    ctx.accounts.market_account.transactions += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitRating<'info>{
    #[account(mut)]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        constraint = 
        (invoker.key() == Pubkey::new(orbit_addresses::PHYSICAL_SIGNER)) ||
        (invoker.key() == Pubkey::new(orbit_addresses::DIGITAL_SIGNER))
    )]
    pub invoker: Signer<'info>,
}

pub fn submit_rating_handler(ctx: Context<SubmitRating>, rating: usize) -> Result<()>{
    ctx.accounts.market_account.reputation[rating] += 1;
    Ok(())
}