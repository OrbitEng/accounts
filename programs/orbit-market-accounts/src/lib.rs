use anchor_lang::prelude::*;

declare_id!("7AwGcaYA8SC32T5kcv5q4u9HhY49a7cNAAp8CpcbibFq");

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
    
    pub fn create_account(ctx: Context<CreateMarketAccount>, pfp_link: String, metadata_link: String) -> Result<()>{
        create_account_handler(ctx, pfp_link, metadata_link)
    }

    pub fn update_profile_image(ctx: Context<UpdateAccountFieldUser>, new_link: String) -> Result<()>{
        update_profile_image_handler(ctx, new_link)
    }

    pub fn post_tx(ctx: Context<PostTxIncrementContext>) -> Result<()>{
        post_tx_handler(ctx)
    }

    pub fn submit_rating(ctx: Context<MarketAccountUpdateInternal>, rating: usize) -> Result<()>{
        submit_rating_handler(ctx, rating)
    }

    ////// DISPUTE
    
    pub fn increment_dispute_discounts(ctx: Context<MarketAccountUpdateInternal>) -> Result<()>{
        increment_dispute_discounts_handler(ctx)
    }

    pub fn decrement_dispute_discounts(ctx: Context<MarketAccountUpdateInternal>) -> Result<()>{
        decrement_dispute_discounts_handler(ctx)
    }

    pub fn increment_dispute_discounts_multiple(ctx: Context<MarketAccountMultipleUpdateInternal>) -> Result<()>{
        increment_dispute_discounts_multiple_handler(ctx)
    }

    ////// REFLINKS
    pub fn set_reflink(ctx: Context<AddReflink>) -> Result<()>{
        add_reflink_handler(ctx)
    }
    pub fn remove_reflink(ctx: Context<RemoveReflink>) -> Result<()>{
        remove_reflink_handler(ctx)
    }

    ////// LISTINGS
    
    pub fn add_vendor_physical_listings(ctx: Context<InitVendorListings>, market_type: String) -> Result<()> {
        add_vendor_physical_listings_handler(ctx, market_type)
    }
    pub fn add_vendor_digital_listings(ctx: Context<InitVendorListings>, market_type: String) -> Result<()> {
        add_vendor_digital_listings_handler(ctx, market_type)
    }
    pub fn add_vendor_commission_listings(ctx: Context<InitVendorListings>, market_type: String) -> Result<()> {
        add_vendor_commission_listings_handler(ctx, market_type)
    }

    ///////// TRANSACTION LOGS
    /// : BUYER
    pub fn add_buyer_physical_transactions(ctx: Context<InitBuyerTransactionsLog>, market_type: String) -> Result<()>{
        add_buyer_physical_transactions_handler(ctx, market_type)
    }
    pub fn add_buyer_digital_transactions(ctx: Context<InitBuyerTransactionsLog>, market_type: String) -> Result<()>{
        add_buyer_digital_transactions_handler(ctx, market_type)
    }
    pub fn add_buyer_commission_transactions(ctx: Context<InitBuyerTransactionsLog>, market_type: String) -> Result<()>{
        add_buyer_commission_transactions_handler(ctx, market_type)
    }

    
    /// :SELLER
    pub fn add_seller_physical_transactions(ctx: Context<InitSellerTransactionsLog>, market_type: String) -> Result<()>{
        add_seller_physical_transactions_handler(ctx, market_type)
    }
    pub fn add_seller_digital_transactions(ctx: Context<InitSellerTransactionsLog>, market_type: String) -> Result<()>{
        add_seller_digital_transactions_handler(ctx, market_type)
    }
    pub fn add_seller_commission_transactions(ctx: Context<InitSellerTransactionsLog>, market_type: String) -> Result<()>{
        add_seller_commission_transactions_handler(ctx, market_type)
    }

    ///////// TRANSFERS
    
    pub fn initiate_transfer(ctx: Context<InitTransfer>) -> Result<()>{
        account_transfer_init(ctx)
    }

    pub fn confirm_transfer(ctx: Context<ConfirmTransfer>) -> Result<()> {
        account_transfer_confirm(ctx)
    }

    pub fn decline_transfer(ctx: Context<DeclineTransfer>) -> Result<()>{
        account_transfer_decline(ctx)
    }

    /////////////////
    /// REFLINK UTILS
    
    pub fn create_reflink(ctx: Context<CreateReflink>) -> Result<()>{
        init_reflink_handler(ctx)
    }

    pub fn delete_reflink(ctx: Context<DeleteReflink>) -> Result<()>{
        delete_reflink_handler(ctx)
    }

    pub fn transfer_reflink(ctx: Context<TransferReflink>) -> Result<()>{
        transfer_reflink_handler(ctx)
    }


    ////////////////////
    /// PROGRAM INIT UTILS
    
    pub fn initialize_voter_struct(ctx: Context<CreateVoterIdStruct>) -> Result<()>{
        initialize_voter_id_handler(ctx)
    }
}