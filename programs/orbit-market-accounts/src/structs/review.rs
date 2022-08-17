use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub struct TransactionReviews{
    pub buyer: bool,
    pub seller: bool
}

#[error_code]
pub enum ReviewErrors{
    #[msg("reviews can only be from 1 to 5")]
    RatingOutsideRange,
    #[msg("You can't provide a review")]
    InvalidReviewAuthority,
}