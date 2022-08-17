use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod accessors;
pub mod structs;
pub mod errors;

pub use accessors::*;
pub use structs::*;
pub use errors::*;

#[program]
pub mod orbit_market_accounts {
    use super::*;

    pub fn create_account(ctx: Context<CreateMarketAccount>, metadata_link: [u8; 64], payer_as_wallet: bool) -> Result<()>{
        create_account_handler(ctx, metadata_link, payer_as_wallet)

    }
    pub fn set_wallet(ctx: Context<SetWallet>) -> Result<()>{
        set_wallet_handler(ctx)

    }
    pub fn post_tx(ctx: Context<IncrementTransactions>) -> Result<()>{
        post_tx_handler(ctx)

    }
    pub fn submit_rating(ctx: Context<SubmitRating>, rating: usize) -> Result<()>{
        submit_rating_handler(ctx, rating)
    }
}