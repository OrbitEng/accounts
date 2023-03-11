use anchor_lang::{prelude::*, AccountsClose};
use crate::{
    AccountTransfer,
    OrbitMarketAccount
};

#[derive(Accounts)]
pub struct InitTransfer<'info>{

    #[account(
        init,
        seeds = [
            b"orbit_transfer",
            source_market_account.key().as_ref(),
            destination_market_account.key().as_ref()
        ],
        bump,
        payer = source_wallet,
        space = 100 // 32 pubkey + 32 pubkey + 8 discriminator. should give more for leeway
    )]
    pub transfer_struct: Account<'info, AccountTransfer>,

    #[account(
        seeds = [
            b"orbit_account",
            source_wallet.key().as_ref()
        ],
        bump,
        constraint = source_market_account.transfer_struct == Pubkey::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])
    )]
    pub source_market_account: Box<Account<'info, OrbitMarketAccount>>,

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
        bump,
        constraint = destination_market_account.transfer_struct == Pubkey::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])
    )]
    pub destination_market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = destination_market_account.wallet
    )]
    pub destination_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

pub fn account_transfer_init(ctx: Context<InitTransfer>) -> Result<()> {
    ctx.accounts.transfer_struct.destination = ctx.accounts.destination_market_account.key();
    ctx.accounts.transfer_struct.source = ctx.accounts.source_market_account.key();

    // we set these to make querying easier from clientside
    ctx.accounts.source_market_account.transfer_struct = ctx.accounts.transfer_struct.key();
    ctx.accounts.destination_market_account.transfer_struct = ctx.accounts.transfer_struct.key();
    Ok(())
}

#[derive(Accounts)]
pub struct ConfirmTransfer<'info>{
    #[account(
        mut,
        seeds = [
            b"orbit_account",
            source_wallet.key().as_ref()
        ],
        bump,
        constraint = source_market_account.transfer_struct == transfer_request.key()
    )]
    pub source_market_account: Box<Account<'info, OrbitMarketAccount>>,

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
        bump,
        constraint = destination_market_account.transfer_struct == transfer_request.key()
    )]
    pub destination_market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = destination_market_account.wallet
    )]
    pub destination_wallet: Signer<'info>,

    #[account(
        seeds = [
            b"orbit_transfer",
            source_market_account.key().as_ref(),
            destination_market_account.key().as_ref()
        ],
        bump,
        constraint = (transfer_request.source == source_market_account.key()) && ((transfer_request.source == destination_market_account.key()))
    )]
    pub transfer_request: Account<'info, AccountTransfer>
}

pub fn account_transfer_confirm(ctx: Context<ConfirmTransfer>) -> Result<()> {
    // transfer all the data
    ctx.accounts.destination_market_account.transactions = ctx.accounts.source_market_account.transactions;
    ctx.accounts.destination_market_account.account_created = ctx.accounts.source_market_account.account_created;
    ctx.accounts.destination_market_account.reputation = ctx.accounts.source_market_account.reputation;
    ctx.accounts.destination_market_account.metadata = ctx.accounts.source_market_account.metadata.clone();
    ctx.accounts.destination_market_account.profile_pic = ctx.accounts.source_market_account.profile_pic.clone();
    ctx.accounts.destination_market_account.used_reflink = ctx.accounts.source_market_account.used_reflink;
    ctx.accounts.destination_market_account.dispute_discounts = ctx.accounts.source_market_account.dispute_discounts;
    ctx.accounts.destination_market_account.voter_id = ctx.accounts.source_market_account.voter_id;
    ctx.accounts.destination_market_account.digital_listings = ctx.accounts.source_market_account.digital_listings;
    ctx.accounts.destination_market_account.physical_listings = ctx.accounts.source_market_account.physical_listings;
    ctx.accounts.destination_market_account.commission_listings = ctx.accounts.source_market_account.commission_listings;
    ctx.accounts.destination_market_account.buyer_digital_transactions = ctx.accounts.source_market_account.buyer_digital_transactions;
    ctx.accounts.destination_market_account.buyer_physical_transactions = ctx.accounts.source_market_account.buyer_physical_transactions;
    ctx.accounts.destination_market_account.buyer_commission_transactions = ctx.accounts.source_market_account.buyer_commission_transactions;
    ctx.accounts.destination_market_account.seller_digital_transactions = ctx.accounts.source_market_account.seller_digital_transactions;
    ctx.accounts.destination_market_account.seller_physical_transactions = ctx.accounts.source_market_account.seller_physical_transactions;
    ctx.accounts.destination_market_account.seller_commission_transactions = ctx.accounts.source_market_account.seller_commission_transactions;
    ctx.accounts.destination_market_account.owned_reflink = ctx.accounts.source_market_account.owned_reflink;

    // close old account to old wallet
    ctx.accounts.source_market_account.transfer_struct = Pubkey::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.source_market_account.close(ctx.accounts.source_wallet.to_account_info()).expect("could not close old market account");
    ctx.accounts.transfer_request.close(ctx.accounts.source_wallet.to_account_info()).expect("could not close transfer struct");
    ctx.accounts.destination_market_account.transfer_struct = Pubkey::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    Ok(())
}

#[derive(Accounts)]
pub struct DeclineTransfer<'info>{
    #[account(
        mut,
        seeds = [
            b"orbit_account",
            source_wallet.key().as_ref()
        ],
        bump,
        constraint = source_market_account.transfer_struct == transfer_request.key()
    )]
    pub source_market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = source_market_account.wallet
    )]
    pub source_wallet: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"orbit_account",
            destination_wallet.key().as_ref()
        ],
        bump,
        constraint = destination_market_account.transfer_struct == transfer_request.key()
    )]
    pub destination_market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = destination_market_account.wallet
    )]
    pub destination_wallet: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"orbit_transfer",
            source_market_account.key().as_ref(),
            destination_market_account.key().as_ref()
        ],
        bump,
        constraint = (transfer_request.source == source_market_account.key()) && ((transfer_request.source == destination_market_account.key()))
    )]
    pub transfer_request: Account<'info, AccountTransfer>,

    #[account(
        constraint = (invoker.key() == destination_wallet.key()) || (invoker.key() == source_wallet.key())
    )]
    pub invoker: Signer<'info>
}

pub fn account_transfer_decline(ctx: Context<DeclineTransfer>) -> Result<()>{
    ctx.accounts.source_market_account.transfer_struct = Pubkey::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.destination_market_account.transfer_struct = Pubkey::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

    ctx.accounts.source_market_account.close(ctx.accounts.source_wallet.to_account_info()).expect("could not close old market account");
    ctx.accounts.transfer_request.close(ctx.accounts.source_wallet.to_account_info())
}