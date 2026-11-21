use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakingPool {
    /// Pool administrator.
    pub authority: Pubkey,

    /// Token users deposit.
    pub staking_mint: Pubkey,

    /// Token users receive as rewards.
    pub reward_mint: Pubkey,

    /// PDA-controlled staking vault.
    pub staking_vault: Pubkey,

    /// PDA-controlled reward vault.
    pub reward_vault: Pubkey,

    /// Reward rate in reward-token base units
    /// per whole staking token per second.
    pub reward_rate: u64,

    /// Total tokens deposited into the pool.
    pub total_staked: u64,

    /// Timestamp of the last global reward update.
    pub last_update: i64,

    /// Accumulated reward per whole staking token,
    /// scaled by REWARD_PRECISION.
    pub reward_per_token_stored: u128,

    /// Pool PDA bump.
    pub bump: u8,
}