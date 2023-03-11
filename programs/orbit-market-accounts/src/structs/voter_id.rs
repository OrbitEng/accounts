use anchor_lang::prelude::*;

#[account]
pub struct VoterId{
    pub current_voters: u64,
}