use anchor_lang::prelude::*;


#[account]
pub struct OrbitMarketAccount{
    // note: pubkey is just [u8; 32]
    //    we're gonna do some cool magic soon >:)
    pub wallet: Pubkey,
    // if someone does more than 4 trillion transactions, ill change this value
    pub transactions: u32,

    pub account_created: i64,

    // THIS IS ON A SCALE OF 0-5 LIKE UBER DONT TRY TO GET SLICK AND GIVE URSELF A 255 I WILL PERSONALLY FIND U AND FUCK U IN THE ASS
    pub reputation: [u32; 5],
    
    // we're gonna allocate this 256 to be UBER FUCKING SAFE
    // ideally from FE we query arweave.net/txid/{metadata}
    //      note: fields that should not be changed (like name) must be here
    //            we dont let ppl change names so they cant scam
    pub metadata: String, // 43
    pub profile_pic: String, // 43

    // reflink account of referrer
    pub reflink: Pubkey,
    pub dispute_discounts: u16,

    pub transfer_struct: Pubkey
}