use std::future::poll_fn;

use anchor_lang::prelude::*;

use crate::constants::REWARD_PRECISION;
use crate::error::StakingError::{self, *};
use crate::states::{StakingPool, UserStake};

pub fn update_pool_reward(
    pool:&mut Account<StakingPool>,
    current_time:i64
) -> Result<()> {

    if current_time <= pool.last_update {
        return Ok(());
    }

    let elapsed = current_time.checked_sub(pool.last_update).ok_or(StakingError::ArithmeticOverflow)?;

    if pool.total_staked > 0 {
        let reward = (pool.reward_rate as u128)
            .checked_mul(elapsed as u128)
            .ok_or(StakingError::ArithmeticOverflow)?;

        let reward_per_token = reward
            .checked_mul(REWARD_PRECISION)
            .ok_or(StakingError::ArithmeticOverflow)?
            .checked_div(pool.total_staked as u128)
            .ok_or(StakingError::ArithmeticOverflow)?;

        pool.reward_per_token_stored = pool.reward_per_token_stored
            .checked_add(reward_per_token)
            .ok_or(StakingError::ArithmeticOverflow)?;
    }

    pool.last_update = current_time;


    Ok(())
}

pub fn update_user_reward(
    user_stake:&mut Account<UserStake>,
    reward_per_token_stored:u128 
) -> Result<()> {

    let accumulated = reward_per_token_stored
        .checked_sub(user_stake.reward_per_token_paid)
        .ok_or(StakingError::ArithmeticOverflow)?;

    let earned = (user_stake.amount as u128)
        .checked_mul(accumulated)
        .ok_or(StakingError::ArithmeticOverflow)?
        .checked_div(REWARD_PRECISION)
        .ok_or(StakingError::ArithmeticOverflow)?;

    let earned = u64::try_from(earned)
        .map_err(|_| StakingError::ArithmeticOverflow)?;

    user_stake.pending_rewards = user_stake.pending_rewards
        .checked_add(earned)
        .ok_or(StakingError::ArithmeticOverflow)?;

    user_stake.reward_per_token_paid = reward_per_token_stored;


    Ok(())
}