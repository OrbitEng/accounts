use anchor_lang::prelude::*;
use crate::{structs::AccountTransfer, OrbitMarketAccount};

#[derive(Accounts)]
pub struct InitTransafer<'info>{

    #[account(
        init,
        seeds = [],
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
}

// tood: implement logic && expose in lib.rs