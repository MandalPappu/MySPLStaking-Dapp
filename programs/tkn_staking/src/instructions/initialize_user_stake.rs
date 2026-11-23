use anchor_lang::prelude::*;

use crate::constants::{USER_STAKE_SEED};
use crate::states::{StakingPool, UserStake};

#[derive(Accounts)]
pub struct InitializeUserStake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub pool: Account<'info, StakingPool>,

    #[account(
        init,
        payer = user,
        space = 8 + UserStake::INIT_SPACE,
        seeds = [USER_STAKE_SEED, pool.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_stake: Account<'info, UserStake>,

    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<InitializeUserStake>) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    user_stake.owner = ctx.accounts.user.key();
    user_stake.amount = 0;
    user_stake.reward_per_token_paid = ctx.accounts.pool.reward_per_token_stored;
    user_stake.pending_rewards = 0;
    user_stake.bump = ctx.bumps.user_stake;

    msg!("User staking account initialized!");
    Ok(())
}