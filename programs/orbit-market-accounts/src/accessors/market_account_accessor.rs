use anchor_lang::prelude::*;
use crate::{
    structs::market_account::OrbitMarketAccount,
    OrbitReflink, VoterId, MarketAccountErrors
};
use orbit_addresses::{
    PHYSICAL_ADDRESS,
    DIGITAL_ADDRESS,
    COMMISSION_ADDRESS, DISPUTE_ADDRESS
};
use orbit_product::program::OrbitProduct;
use orbit_transaction::program::OrbitTransaction;

////////////////////////////////////////////////
/// GENERAL

#[derive(Accounts)]
pub struct CreateMarketAccount<'info>{
    #[account(
        init,
        space = 800, // metadata should be of len 44. profile pic url is variable
        payer = wallet,
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        seeds = [
            b"orbit_voters"
        ],
        bump
    )]
    pub voter_id_struct: Account<'info, VoterId>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_account_handler(ctx: Context<CreateMarketAccount>, pfp_link: String, metadata_link: String) -> Result<()>{
    let clock = Clock::get().expect("Could not get CLOCK SYSVAR");

    ctx.accounts.market_account.wallet = ctx.accounts.wallet.key();
    ctx.accounts.market_account.account_created = clock.unix_timestamp;
    ctx.accounts.market_account.metadata = metadata_link;
    ctx.accounts.market_account.profile_pic = pfp_link;

    ctx.accounts.market_account.voter_id = ctx.accounts.voter_id_struct.current_voters;
    ctx.accounts.voter_id_struct.current_voters += 1;

    // 人之初，性本善。性相近，习相远
    ctx.accounts.market_account.reputation = [0; 5];
    ctx.accounts.market_account.transactions = 0;
    ctx.accounts.market_account.owned_reflink = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.market_account.transfer_struct = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.market_account.digital_listings = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.market_account.physical_listings = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.market_account.commission_listings = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.market_account.dispute_discounts = 0;

    if ctx.remaining_accounts.len() == 1{
        let mut reflink_acc = Account::<OrbitReflink>::try_from(&ctx.remaining_accounts[0].to_account_info()).expect("reflink does not exist");
        ctx.accounts.market_account.used_reflink = reflink_acc.key();
        reflink_acc.uses += 1;
        reflink_acc.exit(ctx.program_id)?;
    }else{
        ctx.accounts.market_account.used_reflink = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    }
    Ok(())
}

/// UTILS

#[derive(Accounts)]
pub struct UpdateAccountFieldUser<'info>{
    #[account(
        mut,
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>
}

pub fn update_profile_image_handler(ctx: Context<UpdateAccountFieldUser>, new_link: String) -> Result<()>{
    ctx.accounts.market_account.profile_pic = new_link;
    Ok(())
}

pub fn update_metadata_handler(ctx: Context<UpdateAccountFieldUser>, new_link: String) -> Result<()>{
    ctx.accounts.market_account.metadata = new_link;
    Ok(())
}

//////////////////////////////////////////////////////////////////////
/// REFLINK

#[derive(Accounts)]
pub struct AddReflink<'info>{
    #[account(
        mut,
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        constraint = reflink.uses < 10
    )]
    pub reflink: Account<'info, OrbitReflink>,

    #[account(
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>
}

pub fn add_reflink_handler(ctx: Context<AddReflink>) -> Result<()>{
    ctx.accounts.market_account.used_reflink = ctx.accounts.reflink.key();
    ctx.accounts.reflink.uses += 1;
    ctx.accounts.reflink.users.push(ctx.accounts.market_account.key());
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReflink<'info>{
    #[account(
        mut,
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = market_account.used_reflink
    )]
    pub reflink: Account<'info, OrbitReflink>,

    #[account(
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>
}

pub fn remove_reflink_handler(ctx: Context<RemoveReflink>) -> Result<()>{
    ctx.accounts.market_account.used_reflink = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.reflink.uses -= 1;
    let pos = ctx.accounts.reflink.users.iter().position(|user| *user == ctx.accounts.market_account.key()).expect("user not found for reflink");
    if pos == (ctx.accounts.reflink.users.len()-1){
        ctx.accounts.reflink.users.drain(pos..);
    }else{
        ctx.accounts.reflink.users.drain(pos..pos+1);
    }
    
    Ok(())
}

//////////////////////////////////////////////////
/// LISTINGS 

#[derive(Accounts)]
#[instruction(market_type: String)]
pub struct InitVendorListings<'info>{
    #[account(
        mut,
        has_one = wallet
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        seeds = [
            b"listings_catalog",
            market_type.as_bytes(),
            wallet.key().as_ref()
        ],
        bump,
        seeds::program = orbit_product::ID
    )]
    pub listings_struct: SystemAccount<'info>,

    #[account(
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>,
    pub product_program: Program<'info, OrbitProduct>,
    pub system_program: Program<'info, System>
}

pub fn add_vendor_physical_listings_handler(ctx: Context<InitVendorListings>, market_type: String) -> Result<()> {
    if market_type != "physical"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    ctx.accounts.market_account.digital_listings = ctx.accounts.listings_struct.key();
    Ok(())
}
pub fn add_vendor_digital_listings_handler(ctx: Context<InitVendorListings>, market_type: String) -> Result<()> {
    if market_type != "digital"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    ctx.accounts.market_account.digital_listings = ctx.accounts.listings_struct.key();
    Ok(())
}
pub fn add_vendor_commission_listings_handler(ctx: Context<InitVendorListings>, market_type: String) -> Result<()> {
    if market_type != "commission"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    ctx.accounts.market_account.digital_listings = ctx.accounts.listings_struct.key();
    Ok(())
}

//////////////////////////////////////////////////
///////// TRANSACTIONS LOGS

/// :BUYER

#[derive(Accounts)]
#[instruction(market_type: String)]
pub struct InitBuyerTransactionsLog<'info>{
    #[account(
        mut,
        has_one = wallet
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        seeds = [
            b"buyer_transactions",
            market_type.as_bytes(),
            wallet.key().as_ref()
        ],
        bump,
        seeds::program = orbit_transaction::ID
    )]
    pub transactions_log: SystemAccount<'info>,

    #[account(
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>,

    pub transactions_program: Program<'info, OrbitTransaction>,
    pub system_program: Program<'info, System>
}

pub fn add_buyer_physical_transactions_handler(ctx: Context<InitBuyerTransactionsLog>, market_type: String) -> Result<()>{
    if market_type != "physical"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    orbit_transaction::cpi::create_buyer_transactions_log(
        CpiContext::new(
            ctx.accounts.transactions_program.to_account_info(),
            orbit_transaction::cpi::accounts::CreateBuyerLog{
                transactions_log: ctx.accounts.transactions_log.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info()
            }
        ),
        market_type
    )?;
    ctx.accounts.market_account.physical_listings = ctx.accounts.transactions_log.key();
    Ok(())
}

pub fn add_buyer_digital_transactions_handler(ctx: Context<InitBuyerTransactionsLog>, market_type: String) -> Result<()>{
    if market_type != "digital"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    orbit_transaction::cpi::create_buyer_transactions_log(
        CpiContext::new(
            ctx.accounts.transactions_program.to_account_info(),
            orbit_transaction::cpi::accounts::CreateBuyerLog{
                transactions_log: ctx.accounts.transactions_log.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info()
            }
        ),
        market_type
    )?;
    ctx.accounts.market_account.digital_listings = ctx.accounts.transactions_log.key();
    Ok(())
}

pub fn add_buyer_commission_transactions_handler(ctx: Context<InitBuyerTransactionsLog>, market_type: String) -> Result<()>{
    if market_type != "commission"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    orbit_transaction::cpi::create_buyer_transactions_log(
        CpiContext::new(
            ctx.accounts.transactions_program.to_account_info(),
            orbit_transaction::cpi::accounts::CreateBuyerLog{
                transactions_log: ctx.accounts.transactions_log.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info()
            }
        ),
        market_type
    )?;
    ctx.accounts.market_account.commission_listings = ctx.accounts.transactions_log.key();
    Ok(())
}

/// :SELLER

#[derive(Accounts)]
#[instruction(market_type: String)]
pub struct InitSellerTransactionsLog<'info>{
    #[account(
        mut,
        has_one = wallet
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        seeds = [
            b"seller_transactions",
            market_type.as_bytes(),
            wallet.key().as_ref()
        ],
        bump,
        seeds::program = orbit_transaction::ID
    )]
    pub transactions_log: SystemAccount<'info>,

    #[account(
        address = market_account.wallet
    )]
    pub wallet: Signer<'info>,
    pub transactions_program: Program<'info, OrbitTransaction>,
    pub system_program: Program<'info, System>
}

pub fn add_seller_physical_transactions_handler(ctx: Context<InitSellerTransactionsLog>, market_type: String) -> Result<()>{
    if market_type != "physical"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    orbit_transaction::cpi::create_seller_transactions_log(
        CpiContext::new(
            ctx.accounts.transactions_program.to_account_info(),
            orbit_transaction::cpi::accounts::CreateSellerLog{
                transactions_log: ctx.accounts.transactions_log.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info()
            }
        ),
        market_type
    )?;
    ctx.accounts.market_account.seller_physical_transactions = ctx.accounts.transactions_log.key();
    Ok(())
}

pub fn add_seller_digital_transactions_handler(ctx: Context<InitSellerTransactionsLog>, market_type: String) -> Result<()>{
    if market_type != "digital"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    orbit_transaction::cpi::create_seller_transactions_log(
        CpiContext::new(
            ctx.accounts.transactions_program.to_account_info(),
            orbit_transaction::cpi::accounts::CreateSellerLog{
                transactions_log: ctx.accounts.transactions_log.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info()
            }
        ),
        market_type
    )?;
    ctx.accounts.market_account.seller_digital_transactions = ctx.accounts.transactions_log.key();
    Ok(())
}

pub fn add_seller_commission_transactions_handler(ctx: Context<InitSellerTransactionsLog>, market_type: String) -> Result<()>{
    if market_type != "commission"{
        return err!(MarketAccountErrors::InvalidSeedString)
    }
    orbit_transaction::cpi::create_seller_transactions_log(
        CpiContext::new(
            ctx.accounts.transactions_program.to_account_info(),
            orbit_transaction::cpi::accounts::CreateSellerLog{
                transactions_log: ctx.accounts.transactions_log.to_account_info(),
                wallet: ctx.accounts.wallet.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info()
            }
        ),
        market_type
    )?;
    ctx.accounts.market_account.seller_commission_transactions = ctx.accounts.transactions_log.key();
    Ok(())
}

/// POST TX CPI'S

#[derive(Accounts)]
pub struct PostTxIncrementContext<'info>{
    #[account(mut)]
    pub buyer_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(mut)]
    pub seller_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        seeds = [
            b"market_authority"
        ],
        bump,
        seeds::program = caller.key()
    )]
    pub caller_auth: Signer<'info>,

    #[account(
        executable,
        constraint = 
            (caller.key().to_bytes() == PHYSICAL_ADDRESS) ||
            (caller.key().to_bytes() == DIGITAL_ADDRESS) ||
            (caller.key().to_bytes() == COMMISSION_ADDRESS)
    )]
    /// CHECK: we do basic checks
    pub caller: AccountInfo<'info>
}

pub fn post_tx_handler(ctx: Context<PostTxIncrementContext>) -> Result<()>{
    ctx.accounts.buyer_account.transactions += 1;
    ctx.accounts.seller_account.transactions += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct MarketAccountUpdateInternal<'info>{
    #[account(mut)]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        seeds = [
            b"market_authority"
        ],
        bump,
        seeds::program = caller.key()
    )]
    pub caller_auth: Signer<'info>,

    #[account(
        executable,
        constraint = 
            (caller.key().to_bytes() == PHYSICAL_ADDRESS) ||
            (caller.key().to_bytes() == DIGITAL_ADDRESS) ||
            (caller.key().to_bytes() == COMMISSION_ADDRESS) ||
            (caller.key().to_bytes() == DISPUTE_ADDRESS)
    )]
    /// CHECK: we do basic checks
    pub caller: AccountInfo<'info>
}

pub fn submit_rating_handler(ctx: Context<MarketAccountUpdateInternal>, rating: usize) -> Result<()>{
    ctx.accounts.market_account.reputation[rating] += 1;
    Ok(())
}

pub fn increment_dispute_discounts_handler(ctx: Context<MarketAccountUpdateInternal>) -> Result<()>{
    ctx.accounts.market_account.dispute_discounts += 1;
    Ok(())
}

pub fn increment_dispute_discounts_multiple_handler(ctx: Context<MarketAccountUpdateInternal>) -> Result<()>{
    for market_acc in ctx.remaining_accounts{
        let mut ma = Account::<OrbitMarketAccount>::try_from(market_acc).expect("could not deserialize remaining account");
        ma.dispute_discounts += 1;
        market_acc.exit(&crate::ID)?;
    }
    ctx.accounts.market_account.dispute_discounts += 1;
    Ok(())
}

pub fn decrement_dispute_discounts_handler(ctx: Context<MarketAccountUpdateInternal>) -> Result<()>{
    ctx.accounts.market_account.dispute_discounts -= 1;
    Ok(())
}