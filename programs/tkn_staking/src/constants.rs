use anchor_lang::prelude::*;


pub const POOL_SEED: &[u8] = b"staking_pool";

pub const STAKING_VAULT_SEED: &[u8] = b"staking_vault";

pub const REWARD_VAULT_SEED: &[u8] = b"reward_vault";

pub const USER_STAKE_SEED: &[u8] = b"user_stake";


/// Reward accounting precision.
pub const REWARD_PRECISION: u128 = 1_000_000_000_000;
