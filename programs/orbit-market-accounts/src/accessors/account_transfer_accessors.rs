use anchor_lang::{prelude::*, AccountsClose};
use crate::{structs::AccountTransfer, OrbitMarketAccount, MarketAccountErrors, OrbitReflink};

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
        return err!(MarketAccountErrors::MismatchedTransferDestination);
    }

    if ctx.accounts.source_market_account.key() != ctx.accounts.transfer_request.source {
        return err!(MarketAccountErrors::MismatchedTransferSource);
    }

    // transfer all the data
    ctx.accounts.source_market_account.transactions = ctx.accounts.source_market_account.transactions;
    ctx.accounts.source_market_account.account_created = ctx.accounts.source_market_account.account_created;
    ctx.accounts.source_market_account.reputation = ctx.accounts.source_market_account.reputation;
    ctx.accounts.source_market_account.metadata = ctx.accounts.source_market_account.metadata.clone();
    ctx.accounts.source_market_account.profile_pic = ctx.accounts.source_market_account.profile_pic.clone();
    ctx.accounts.source_market_account.reflink = ctx.accounts.source_market_account.reflink;
    ctx.accounts.source_market_account.dispute_discounts = ctx.accounts.source_market_account.dispute_discounts;

    if ctx.remaining_accounts.len() == 1{
        let mut reflink = Account::<OrbitReflink>::try_from(&ctx.remaining_accounts[0]).expect("did not pass in a reflink account");
        if (reflink.owner != ctx.accounts.source_market_account.key()) || (ctx.accounts.source_market_account.owned_reflink != ctx.remaining_accounts[0].key()){
            return err!(MarketAccountErrors::MismatchedReflink);
        }

        reflink.owner = ctx.accounts.destination_market_account.key();
        ctx.accounts.destination_market_account.owned_reflink = ctx.accounts.source_market_account.owned_reflink;
    }

    // close old account to old wallet
    ctx.accounts.source_market_account.close(ctx.accounts.source_wallet.to_account_info()).expect("could not close old market account");
    ctx.accounts.transfer_request.close(ctx.accounts.source_wallet.to_account_info())
}
