use anchor_lang::prelude::*;
use orbit_catalog::{
    cpi::{
        accounts::{
            CreateParty,
            ModifyParty
        },
        create_party,
        edit_party
    },
    program::OrbitCatalog,
    OrbitCatalogErrors, OrbitPartyGroup
};

use crate::{
    OrbitMarketAccount,
    MarketAccountErrors, program::OrbitMarketAccounts
};

#[derive(Accounts)]
pub struct InitTopVendorsParty<'info>{

    #[account(
        seeds = [
            b"top_vendors"
        ],
        bump
    )]
    pub top_vendors: SystemAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            b"market_auth"
        ],
        bump
    )]
    pub accounts_auth: SystemAccount<'info>,

    pub catalog_program: Program<'info, OrbitCatalog>,

    pub system_program: Program<'info, System>
}

pub fn init_top_vendor_handler(ctx: Context<InitTopVendorsParty>) -> Result<()>{
    match ctx.bumps.get("accounts_auth"){
        Some(auth_bump) => create_party(
            CpiContext::new_with_signer(
                ctx.accounts.catalog_program.to_account_info(),
                CreateParty{
                    party: ctx.accounts.top_vendors.to_account_info(),
                    auth: ctx.accounts.accounts_auth.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info()
                },
            &[&[b"market_auth", &[*auth_bump]]])
        ),
        None => return err!(OrbitCatalogErrors::CouldNotCallCatalogProgram)
    }
}

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct UpdateTopVendors<'info>{

    #[account(mut)]
    pub top_vendors: Account<'info, OrbitPartyGroup>,

    #[account(
        seeds = [
            b"market_auth"
        ],
        bump
    )]
    pub accounts_auth: SystemAccount<'info>,

    #[account(
        address = top_vendors.accounts[index as usize]
    )]
    pub old_account: Account<'info, OrbitMarketAccount>,

    pub new_account: Account<'info, OrbitMarketAccount>,

    pub catalog_program: Program<'info, OrbitCatalog>,

    pub accounts_program: Program<'info, OrbitMarketAccounts>
}

pub fn update_top_vendors_handler(ctx: Context<UpdateTopVendors>, index: u8) -> Result<()>{
    if ctx.accounts.new_account.transactions < ctx.accounts.old_account.transactions{
        return err!(MarketAccountErrors::InvalidAccountsProgramAction)
    }

    match ctx.bumps.get("accounts_auth"){
        Some(auth_bump) => edit_party(
                CpiContext::new_with_signer(
                    ctx.accounts.catalog_program.to_account_info(),
                    ModifyParty{
                        party: ctx.accounts.top_vendors.to_account_info(),
                        auth: ctx.accounts.accounts_auth.to_account_info(),
                        insertion_acc: ctx.accounts.new_account.to_account_info(),
                        accounts_address: ctx.accounts.accounts_program.to_account_info()
                    },
                    &[&[b"market_auth", &[*auth_bump]]]
                ),
                index
            ),
        None => return err!(OrbitCatalogErrors::CouldNotCallCatalogProgram)
    }
}