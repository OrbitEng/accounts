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
}