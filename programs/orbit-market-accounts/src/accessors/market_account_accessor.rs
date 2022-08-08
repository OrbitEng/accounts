use std::convert::TryInto;

use anchor_lang::prelude::*;
use crate::{
    structs::market_account::{
        MarketAccountFields,
        OrbitMarketAccount
    },
    errors::MarketAccountErrors
};

#[derive(Accounts)]
pub struct ChangeField<'info>{

    #[account(mut)]
    pub market_account: Account<'info, OrbitMarketAccount>,

    // this is the master key I was talking about
    // frontend is just a keypair wrapped in pseudo wallet
    // since we have no way to programmatically EdDSA verify within a program
    // unless we ran a validator or built a geyser plugin or whatever
    #[account(
        constraint = change_authority.key() == market_account.master_pubkey
    )]
    pub change_authority: Signer<'info>
}

pub fn change_field_handler(ctx: Context<ChangeField>, field: MarketAccountFields, value: Vec<u8>) -> Result<()>{
    match field{
        MarketAccountFields::Wallet => {
            let set: [u8; 32];
            match value.try_into(){
                Ok(e) => set = e,
                Err(_) => return err!(MarketAccountErrors::ValueDeserializationError)
            }
            ctx.accounts.market_account.wallet = Pubkey::new_from_array(set);
        },
        MarketAccountFields::Transaction => {},
        MarketAccountFields::Metadata => {
            let set: [u8; 256];
            match value.try_into(){
                Ok(e) => set = e,
                Err(_) => return err!(MarketAccountErrors::ValueDeserializationError)
            }
            ctx.accounts.market_account.metadata = set;
        }
    }
    Ok(())
}