use anchor_lang::prelude::*;
use MarketAccountErrors;
use crate::{structs::AccountTransfer, OrbitMarketAccount};

#[derive(Accounts)]
pub struct InitTransfer<'info>{

    #[account(
        init,
        seeds = [
            b"orbit_transfer",
            source_wallet.key().as_ref(),
            destination_wallet.key().as_ref()
        ],
        bump,
        payer = source_wallet,
        space = 72 // 32 pubkey + 32 pubkey + 8 discriminator. should give more for leeway
    )]
    pub transfer_struct: Account<'info, AccountTransfer>,

    #[account(
        seeds = [
            b"orbit_account",
            source_wallet.key().as_ref()
        ],
        bump
    )]
    pub source_market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = source_market_account.wallet
    )]
    pub source_wallet: Signer<'info>,

    #[account(
        seeds = [
            b"orbit_account",
            destination_wallet.key().as_ref()
        ],
        bump
    )]
    pub destination_market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = destination_market_account.wallet
    )]
    pub destination_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ConfirmTransfer<'info>{
    #[account(
        seeds = [
            b"orbit_account",
            source_wallet.key().as_ref()
        ],
        bump
    )]
    pub source_market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = source_market_account.wallet
    )]
    pub source_wallet: SystemAccount<'info>,

    #[account(
        seeds = [
            b"orbit_account",
            destination_wallet.key().as_ref()
        ],
        bump
    )]
    pub destination_market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = destination_market_account.wallet
    )]
    pub destination_wallet: Signer<'info>,

    #[account(
        seeds = [
            b"orbit_transfer",
            source_market_account.key().as_ref(),
            destination_market_account.key().as_ref()
        ],
        bump
    )]
    pub transfer_request: Account<'info, AccountTransfer>
}

pub fn account_transfer_init(ctx: Context<InitTransfer>) -> Result<()> {
    ctx.accounts.transfer_struct.destination = ctx.accounts.destination_market_account.key();
    ctx.accounts.transfer_struct.source = ctx.accounts.source_market_account.key();

    Ok(())
}

pub fn account_tranfer_confirm(ctx: Context<ConfirmTransfer>) -> Result<()> {
    // check for scam-ish data
    if ctx.accounts.transfer_request.destination != ctx.accounts.destination_market_account.key() {
        err!(MarketAccountErrors::MismatchedTransferDestination);
    }

    if ctx.accounts.source_market_account.key() != ctx.accounts.transfer_request.source {
        err!(MarketAccountErrors::MismatchedTransferSource);
    }

    // transfer all the data
    ctx.accounts.source_market_account.transactions = ctx.accounts.source_market_account.transactions;
    ctx.accounts.source_market_account.account_created = ctx.accounts.source_market_account.account_created;
    ctx.accounts.source_market_account.reputation = ctx.accounts.source_market_account.reputation;
    ctx.accounts.source_market_account.metadata = ctx.accounts.source_market_account.metadata;
    ctx.accounts.source_market_account.profile_pic = ctx.accounts.source_market_account.profile_pic;
    ctx.accounts.source_market_account.reflink = ctx.accounts.source_market_account.reflink;
    ctx.accounts.source_market_account.dispute_discounts = ctx.accounts.source_market_account.dispute_discounts;

    // delete data
    // TODO: check if all this data is set as null the right way
    ctx.accounts.source_market_account.transactions = 0;
    ctx.accounts.source_market_account.account_created = 0;
    ctx.accounts.source_market_account.reputation = [0;5];
    ctx.accounts.source_market_account.metadata = "used".to_string();
    ctx.accounts.source_market_account.profile_pic = "na".to_string();
    // this is def hacky
    ctx.accounts.source_market_account.reflink = ctx.accounts.source_wallet.key();
    ctx.accounts.source_market_account.dispute_discounts = 0;


    Ok(())
}
