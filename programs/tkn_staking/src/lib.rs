use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod rewards;

pub use instructions::*;


declare_id!("PniJcjAqxmSQbDARAWZ6pYAjCHPBa2QiFpKfiqd4pYN");

#[program]
pub mod tkn_staking {

    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64) -> Result<()> {
        instructions::initialize_pool::handler(ctx, reward_rate) 
    }

    pub fn initialize_vaults(ctx: Context<InitializeVaults>) -> Result<()> {
        instructions::initialize_vaults::handler(ctx)
    }
    
    pub fn initialize_user_stake(ctx: Context<InitializeUserStake>) -> Result<()> {
        instructions::initialize_user_stake::handler(ctx)
    }
    pub fn initialize_stake(ctx: Context<Stake>, amount:u64) -> Result<()> {
        instructions::stake::handler(ctx, amount)
    }
    pub fn initialize_unstake(ctx: Context<Unstake>, amount:u64) -> Result<()> {
        instructions::unstake::handler(ctx, amount)
    }
}
