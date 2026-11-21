use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};


use crate::states::StakingPool;
use crate::constants::POOL_SEED;
use crate::error::StakingError;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub staking_mint: Account<'info, Mint>,
    pub reward_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = 8 + StakingPool::INIT_SPACE,
        seeds = [POOL_SEED, staking_mint.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, StakingPool>,

    /// CHECK: Validated using PDA seeds.
    #[account(
        seeds = [b"token_authority", pool.key().as_ref()],
        bump
    )]
    pub token_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


pub(crate) fn handler(ctx: Context<InitializePool>, reward_rate:u64) -> Result<()> {

    require!(reward_rate > 0, StakingError::InvalidRewardRate);

    let pool = &mut ctx.accounts.pool;
    pool.authority = ctx.accounts.authority.key();
    pool.staking_mint = ctx.accounts.staking_mint.key();
    pool.reward_mint = ctx.accounts.reward_mint.key();
    
    // Vaults will be initialized separately.
    pool.staking_vault = Pubkey::default();
    pool.reward_vault = Pubkey::default();
    pool.reward_rate = reward_rate;
    pool.total_staked = 0;
    pool.last_update = Clock::get()?.unix_timestamp;
    pool.reward_per_token_stored = 0;

    pool.bump = ctx.bumps.pool;

    msg!("Staking pool initialized successfully!");
    Ok(())
}
