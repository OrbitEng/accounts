use anchor_lang::prelude::*;

#[error_code]
pub enum MarketAccountErrors{
    #[msg("this pubkey is already used by a non-system program. please generate another one")]
    InvalidMasterPubkey,
    #[msg("Could not call orbit accounts program")]
    CannotCallOrbitAccountsProgram,
    #[msg("invalid action")]
    InvalidAccountsProgramAction,
    #[msg("mismatched destination addresses in transfer request confirmation. This request probably wasn't for you.")]
    MismatchedTransferDestination,
    #[msg("mismatched sources addresses in transfer request confirmation.")]
    MismatchedTransferSource,
    #[msg("reflink passed was not for the source market account")]
    MismatchedReflink,
    #[msg("users passed do not match reflink")]
    MismatchedUsersForReflink,
    #[msg("invalid seed string for struct")]
    InvalidSeedString
}

#[error_code]
pub enum ReviewErrors{
    #[msg("reviews can only be from 1 to 5")]
    RatingOutsideRange,
    #[msg("You can't provide a review")]
    InvalidReviewAuthority,
}
