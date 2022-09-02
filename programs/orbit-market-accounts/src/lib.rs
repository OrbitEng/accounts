use anchor_lang::prelude::*;

declare_id!("ibRC94BJf9AjLN75GwRKkMB6FXFLrhQj2JCC719JMJ4");

pub mod accessors;
pub mod structs;
pub mod errors;

pub use accessors::*;
pub use structs::*;
pub use errors::*;

#[program]
pub mod orbit_market_accounts {
    use super::*;

    pub fn create_account(ctx: Context<CreateMarketAccount>, metadata_link: String, payer_as_wallet: bool) -> Result<()>{
        create_account_handler(ctx, metadata_link, payer_as_wallet)
    }

    pub fn update_profile_image(ctx: Context<UpdateAccountFieldUser>, new_link: String) -> Result<()>{
        update_profile_image_handler(ctx, new_link)
    }

    pub fn set_wallet(ctx: Context<SetWallet>) -> Result<()>{
        set_wallet_handler(ctx)

    }
    pub fn post_tx(ctx: Context<PostTxContext>) -> Result<()>{
        post_tx_handler(ctx)

    }
    pub fn submit_rating(ctx: Context<PostTxContext>, rating: usize) -> Result<()>{
        submit_rating_handler(ctx, rating)
    }
}