use anchor_lang::prelude::*;

#[error_code]
pub enum MarketAccountErrors{
    #[msg("this pubkey is already used by a non-system program. please generate another one")]
    InvalidMasterPubkey
}