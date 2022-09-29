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

    ////////////////////////////////////
    /// ACCOUNT UTILS
    
    pub fn create_account(ctx: Context<CreateMarketAccount>, metadata_link: String) -> Result<()>{
        create_account_handler(ctx, metadata_link)
    }

    pub fn update_profile_image(ctx: Context<UpdateAccountFieldUser>, new_link: String) -> Result<()>{
        update_profile_image_handler(ctx, new_link)
    }

    pub fn post_tx(ctx: Context<PostTxContext>) -> Result<()>{
        post_tx_handler(ctx)
    }

    pub fn set_reflink(ctx: Context<AddReflink>) -> Result<()>{
        add_reflink_handler(ctx)
    }

    pub fn submit_rating(ctx: Context<PostTxContext>, rating: usize) -> Result<()>{
        submit_rating_handler(ctx, rating)
    }
    /////////////////
    /// REFLINK UTILS
    
    pub fn create_reflink(ctx: Context<CreateReflink>) -> Result<()>{
        init_reflink_handler(ctx)
    }
}