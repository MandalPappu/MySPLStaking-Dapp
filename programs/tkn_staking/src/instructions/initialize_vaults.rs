use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::states::StakingPool;


#[derive(Accounts)]
pub struct InitializeVaults<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        constraint = pool.staking_vault == Pubkey::default(),
        constraint = pool.reward_vault == Pubkey::default()
    )]
    pub pool: Account<'info, StakingPool>,

    #[account(
        address = pool.staking_mint,
    )]
    pub staking_mint: Account<'info, Mint>,

    #[account(
        address = pool.reward_mint,
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = staking_mint,
        token::authority = pool,
    )]
    pub staking_vault: Account<'info, TokenAccount>,


    #[account(
        init,
        payer = authority,
        token::mint = reward_mint,
        token::authority = pool,
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub(crate) fn handler(ctx: Context<InitializeVaults>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.staking_vault = ctx.accounts.staking_vault.key();
    pool.reward_vault = ctx.accounts.reward_vault.key();

    msg!("Staking vault initialized!");

    msg!("Reward vault initialized!");

    Ok(())
}