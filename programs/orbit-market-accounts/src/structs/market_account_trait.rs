use anchor_lang::prelude::*;

pub trait OrbitMarketAccountTrait<'a, T>
    where T: Accounts<'a>
{
    fn leave_review(ctx: Context<T>, rating: u8) -> Result<()>;
}