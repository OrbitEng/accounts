use anchor_lang::prelude::*;


#[account]
pub struct OrbitMarketAccount{
    // note: pubkey is just [u8; 32]
    //    we're gonna do some cool magic soon >:)
    pub wallet: Pubkey, // 32

    // 12
    // if someone does more than 4 trillion transactions, ill change this value
    pub transactions: u32,
    pub account_created: i64,

    // 28
    // THIS IS ON A SCALE OF 0-5 LIKE UBER DONT TRY TO GET SLICK AND GIVE URSELF A 255 I WILL PERSONALLY FIND U AND FUCK U IN THE ASS
    pub reputation: [u32; 5],
    pub voter_id: u64,
    
    // 84
    pub metadata: String, // 43
    pub profile_pic: String, // 43

    // reflink account of referrer // 33
    pub reflink: Pubkey,
    pub dispute_discounts: u8,

    // for transferring and shit // 64
    pub owned_reflink: Pubkey,

    pub transfer_struct: Pubkey,

    // 300
    pub digital_vendor_catalog: Pubkey,
    pub physical_vendor_catalog: Pubkey,
    pub commission_vendor_catalog: Pubkey,

    pub buyer_open_digital_transactions: Pubkey,
    pub buyer_open_physical_transactions: Pubkey,
    pub buyer_open_commission_transactions: Pubkey,

    pub seller_open_digital_transactions: Pubkey,
    pub seller_open_physical_transactions: Pubkey,
    pub seller_open_commission_transactions: Pubkey,
}