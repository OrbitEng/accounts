use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub struct TransactionReviews{
    pub buyer: bool,
    pub seller: bool
}