use anchor_lang::prelude::*;

#[account]
pub struct OrbitReflink{
    pub reflink_owner: Pubkey, //32
    pub uses: u8, //1
    pub users: Vec<Pubkey>
}