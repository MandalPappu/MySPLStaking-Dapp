use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Invalid reward rate.")]
    InvalidRewardRate,

    #[msg("Amount must be greater than zero.")]
    InvalidAmount,

    #[msg("Insufficient staked balance.")]
    InsufficientStake,

    #[msg("Arithmetic overflow.")]
    ArithmeticOverflow,

    #[msg("Unauthorized access.")]
    Unauthorized,

    #[msg("Insufficient reward balance.")]
    InsufficientRewards,

    #[msg("Invalid staking pool.")]
    InvalidPool,

    #[msg("Invalid token mint.")]
    InvalidMint,

    #[msg("Cannot claim zero rewards.")]
    NoRewardsAvailable,
}
