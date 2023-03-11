use anchor_lang::prelude::*;

#[account]
pub struct AccountTransfer{
    pub source: Pubkey, // market account address of initiator
    pub destination: Pubkey, // market account address of recipient
}