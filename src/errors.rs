// programs/num-token/src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Contract calls are temporarly suspended")]
    ContractIsPaused,

    #[msg("New admin is the same with old admin")]
    SameAdmin,

    #[msg("New verifier is the same with old verifier")]
    SameVerifier,

    #[msg("Invalid pubkey")]
    InvalidPubKey,

    #[msg("Generation price less than min")]
    GenerationPriceLessThanMin,

    #[msg("Percentage sum must be smaller than 1000 and bigger than 3")]
    InvalidPercentages,

    #[msg("User is already blacklisted")]
    UserIsAlreadyBlackListed,

    #[msg("User is blacklisted")]
    UserIsBlackListed,

    #[msg("Cannot generate without a valid referral 1")]
    InvalidReferral,

    #[msg("Referral 2 must be different than referral 1")]
    InvalidReferral2,

    #[msg("Sender is not an EOA")]
    SenderNotEOA,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Value provided is less than the minimum required")]
    ValueTooLow,

    #[msg("Wrong slippage value")]
    WrongSlippage,

    #[msg("Deadline is passed already")]
    DeadlinePassed,

    #[msg("C8ntinuum usd price oracle must be greater than 0")]
    InvalidPriceOracle,

    #[msg("Price higher than slippage")]
    InvalidPriceOraclePriceHigher,

    #[msg("Price lower than slippage")]
    InvalidPriceOraclePriceLower,

    #[msg("Invalid payload signature")]
    WrongSignature,
}
