use anchor_lang::prelude::*;

#[error_code]
pub enum MarketAccountErrors{
    #[msg("said field for market account was not found")]
    FieldNotFound,
    #[msg("could not deserialize into the value you want to set")]
    ValueDeserializationError
}