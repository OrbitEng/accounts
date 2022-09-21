use anchor_lang::{prelude::*, system_program};
use crate::{
    structs::market_account::OrbitMarketAccount,
    errors::MarketAccountErrors, OrbitReflink
};
use orbit_addresses::{
    PHYSICAL_ADDRESS,
    DIGITAL_ADDRESS
};

#[derive(Accounts)]
pub struct CreateMarketAccount<'info>{
    #[account(
        init,
        space = 400, // metadata should be of len 44. profile pic url is variable
        payer = payer
    )]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub master_auth: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_account_handler(ctx: Context<CreateMarketAccount>, metadata_link: String, payer_as_wallet: bool) -> Result<()>{
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

    if ctx.remaining_accounts.len() == 1{
        let mut reflink_acc = Account::<OrbitReflink>::try_from(&ctx.remaining_accounts[0].to_account_info()).expect("reflink does not exist");
        ctx.accounts.market_account.reflink = reflink_acc.key();
        reflink_acc.uses += 1;
        reflink_acc.exit(ctx.program_id)?;
    }else{
        ctx.accounts.market_account.reflink = Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    }
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
pub struct AddReflink<'info>{
    #[account(mut)]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        constraint = reflink.uses < 10
    )]
    pub reflink: Account<'info, OrbitReflink>,

    #[account(
        address = market_account.master_pubkey
    )]
    pub change_authority: Signer<'info>
}

pub fn add_reflink_handler(ctx: Context<AddReflink>) -> Result<()>{
    ctx.accounts.market_account.reflink = ctx.accounts.reflink.key();
    ctx.accounts.reflink.uses += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateAccountFieldUser<'info>{
    #[account(mut)]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = market_account.master_pubkey
    )]
    pub change_authority: Signer<'info>
}

pub fn update_profile_image_handler(ctx: Context<UpdateAccountFieldUser>, new_link: String) -> Result<()>{
    ctx.accounts.market_account.profile_pic = new_link;
    Ok(())
}

#[derive(Accounts)]
pub struct PostTxContext<'info>{
    #[account(mut)]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        seeds = [
            b"market_authority"
        ],
        seeds::program = caller.key(),
        bump
    )]
    pub caller_auth: Signer<'info>,

    #[account(
        constraint = 
            (caller.key() == Pubkey::new(PHYSICAL_ADDRESS)) ||
            (caller.key() == Pubkey::new(DIGITAL_ADDRESS))
    )]
    /// CHECK: we do do checks
    pub caller: AccountInfo<'info>
}

pub fn post_tx_handler(ctx: Context<PostTxContext>) -> Result<()>{
    ctx.accounts.market_account.transactions += 1;
    Ok(())
}

pub fn submit_rating_handler(ctx: Context<PostTxContext>, rating: usize) -> Result<()>{
    ctx.accounts.market_account.reputation[rating] += 1;
    Ok(())
}