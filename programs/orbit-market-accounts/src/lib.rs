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

    pub fn change_account_field(ctx: Context<ChangeField>, field: MarketAccountFields, value: Vec<u8>) -> Result<()>{
        change_field_handler(ctx, field, value)
    }
}