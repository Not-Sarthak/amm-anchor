use anchor_lang::error_code;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("Fee Percentage can only be between 0 to 100 (10,000 basis points)")]
    FeePercentErr,
    #[msg("DefaultError")]
    DefaultError,
    #[msg("Offer Expired.")]
    OfferExpired,
    #[msg("This Pool is Locked.")]
    PoolLocked,
    #[msg("Slippage Exceeded.")]
    SlippageExceeded,
    #[msg("Overflow Detected.")]
    Overflow,
    #[msg("Underflow Detected.")]
    Underflow,
    #[msg("Invalid Token.")]
    InvalidToken,
    #[msg("Actual Liquidity is Less than Minimum.")]
    LiquidityLessThanMinimum,
    #[msg("No Liquidity in Pool.")]
    NoLiquidityInPool,
    #[msg("Bump Error.")]
    BumpError,
    #[msg("Curve Error.")]
    CurveError,
    #[msg("Fee is Greater than 100%. This is not a very good deal.")]
    InvalidFee,
    #[msg("Invalid update authority.")]
    InvalidAuthority,
    #[msg("No update authority set.")]
    NoAuthoritySet,
    #[msg("Invalid Amount.")]
    InvalidAmount,
    #[msg("Invalid Precision.")]
    InvalidPrecision,
    #[msg("Insufficient Balance.")]
    InsufficientBalance,
    #[msg("Zero Balance.")]
    ZeroBalance,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::Overflow,
            CurveError::Underflow => AmmError::Underflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFee,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::ZeroBalance => AmmError::ZeroBalance,
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
        }
    }
}