use anchor_lang::prelude::*;

#[account]
pub struct OrbitReflink{
    pub uses: u8, //1
    pub owner: Pubkey, //32
    pub users: Vec<Pubkey>
}