use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct UserStake {
    // wallet that owns user stake account
    pub owner:Pubkey,
    pub amount:u64,
    pub reward_per_token_paid:u128,
    pub pending_rewards:u64,
    pub bump:u8
}