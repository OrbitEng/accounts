use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum MarketAccountFields{
    Wallet,
    Transaction,
    Metadata
}

#[account]
pub struct OrbitMarketAccount{
    // note: pubkey is just [u8; 32]
    //    we're gonna do some cool magic soon >:)
    pub wallet: Pubkey,
    // if this isnt enough, someone has done over 4 trillion transactions
    // and they probably have the funds to just build a new market or whatever
    pub transactions: u32,
    // we're gonna allocate this 256 to be UBER FUCKING SAFE
    // ideally from FE we query arweave.net/txid/{metadata}
    //      note: fields that should not be changed (like name) must be here
    //            we dont let ppl change names so they cant scam
    pub metadata: [u8; 256],

    // I want this to be the pubkey that the owner uses to make other changes
    // like they should be able to link diff wallets, but if their original wallet isnt connect
    // how do we know they have XYZ authority? or if a wallet isnt connected at all, there's nothing
    // to sign for.
    pub master_pubkey: Pubkey,
}