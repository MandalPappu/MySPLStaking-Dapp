use anchor_lang::prelude::*;

use anchor_spl::token::{
    self,
    Token,
    TokenAccount,
    Transfer,
};

use crate::constants::{
    POOL_SEED,
};

use crate::error::StakingError;

use crate::rewards::{
    update_pool_reward,
    update_user_reward,
};

use crate::states::{
    StakingPool,
    UserStake,
};

#[derive(Accounts)]
pub struct Unstake<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            POOL_SEED,
            pool.staking_mint.as_ref()
        ],
        bump = pool.bump,
        constraint = pool.staking_vault == staking_vault.key()
            @ StakingError::InvalidPool
    )]
    pub pool: Account<'info, StakingPool>,

    #[account(
        mut,
        seeds = [
            b"user_stake",
            pool.key().as_ref(),
            user.key().as_ref()
        ],
        bump = user_stake.bump,
        constraint = user_stake.owner == user.key()
            @ StakingError::Unauthorized
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key()
            @ StakingError::Unauthorized,
        constraint = user_token_account.mint == pool.staking_mint
            @ StakingError::InvalidMint
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = pool.staking_vault,
        constraint = staking_vault.mint == pool.staking_mint
            @ StakingError::InvalidMint,
        constraint = staking_vault.owner == pool.key()
            @ StakingError::InvalidPool
    )]
    pub staking_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub(crate) fn handler(
    ctx: Context<Unstake>,
    amount: u64,
) -> Result<()> {

    // 1. Validate amount.
    require!(
        amount > 0,
        StakingError::InvalidAmount
    );

    // 2. Validate user balance.
    require!(
        ctx.accounts.user_stake.amount >= amount,
        StakingError::InsufficientStake
    );

    let clock = Clock::get()?;

    // 3. Update global rewards.
    update_pool_reward(
        &mut ctx.accounts.pool,
        clock.unix_timestamp,
    )?;

    // 4. Update user rewards before reducing stake.
    update_user_reward(
        &mut ctx.accounts.user_stake,
        ctx.accounts.pool.reward_per_token_stored,
    )?;

    // 5. Derive PDA signer seeds.
    let staking_mint = ctx.accounts.pool.staking_mint;

    let bump = [ctx.accounts.pool.bump];

    let seeds: &[&[u8]] = &[
        POOL_SEED,
        staking_mint.as_ref(),
        &bump,
    ];

    let signer_seeds:&[&[&[u8]]] = &[seeds];

    // 6. Prepare token transfer.
    let cpi_accounts = Transfer {
        from: ctx.accounts.staking_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        cpi_accounts,
        signer_seeds,
    );

    // 7. Transfer tokens from vault to user.
    token::transfer(
        cpi_context,
        amount,
    )?;

    // 8. Update user staking balance.
    let user_stake = &mut ctx.accounts.user_stake;

    user_stake.amount = user_stake
        .amount
        .checked_sub(amount)
        .ok_or(StakingError::ArithmeticOverflow)?;

    // 9. Update total pool stake.
    let pool = &mut ctx.accounts.pool;

    pool.total_staked = pool
        .total_staked
        .checked_sub(amount)
        .ok_or(StakingError::ArithmeticOverflow)?;

    msg!("Unstaking successful!");

    msg!("Withdrawn amount: {}", amount);

    msg!("Remaining stake: {}", user_stake.amount);

    Ok(())
}