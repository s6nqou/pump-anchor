use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
    #[msg("The given account is not authorized to execute this instruction.")]
    NotAuthorized,

    #[msg("The program is already initialized.")]
    AlreadyInitialized,

    #[msg("slippage: Too much SOL required to buy the given amount of tokens.")]
    TooMuchSolRequired,

    #[msg("slippage: Too little SOL received to sell the given amount of tokens.")]
    TooLittleSolReceived,

    #[msg("The mint does not match the bonding curve.")]
    MintDoesNotMatchBondingCurve,

    #[msg("The bonding curve has completed and liquidity migrated to raydium.")]
    BondingCurveComplete,

    #[msg("The bonding curve has not completed.")]
    BondingCurveNotComplete,

    #[msg("The program is not initialized.")]
    NotInitialized,
}
