use anchor_lang::prelude::*;

#[account]
pub struct OrbitReflink{
    pub uses: u8,
}