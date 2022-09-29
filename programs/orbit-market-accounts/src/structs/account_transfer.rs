use anchor_lang::prelude::*;

#[accounts]
pub struct AccountTransfer{
    pub source: Pubkey, // market account address of initiator
    pub destination: Pubkey, // market account address of recipient
}