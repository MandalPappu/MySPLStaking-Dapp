use {
    anchor_lang::{
        prelude::Pubkey,
        AccountDeserialize
    },
    litesvm::LiteSVM,
    tkn_staking::{
        constants::{POOL_SEED, USER_STAKE_SEED},
        states::{StakingPool},
        ID as PROGRAM_ID
    }
};



// =====================================================
// 1. DERIVE POOL PDA
// =====================================================


pub fn derive_pool_PDA (staking_mint:&Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[POOL_SEED, staking_mint.as_ref()], &PROGRAM_ID)
}


// =====================================================
// 2. DERIVE TOKEN AUTHORITY PDA
// =====================================================

pub fn derive_token_authority_PDA (pool:&Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"token_authority", pool.as_ref()], &PROGRAM_ID)
}

// =====================================================
// 3. DERIVE USER STAKE PDA
// =====================================================

pub fn derive_user_stake_PDA(
    pool: &Pubkey,
    user: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            USER_STAKE_SEED,
            pool.as_ref(),
            user.as_ref(),
        ],
        &PROGRAM_ID,
    )
}

// =====================================================
// 4. GET POOL STATE
// =====================================================

pub fn get_pool_state(
    svm:&mut LiteSVM,
    pool: &Pubkey
) -> StakingPool {
    let account = svm.get_account(pool).expect("pool account not found!");

    let mut data:&[u8] = &account.data;

    StakingPool::try_deserialize(&mut data).expect("failed to deserialize staking Pool account!")
}


// =====================================================
// 5. GET USER STAKE STATE
// =====================================================

// pub fn get_user_stake_state(
//     svm: &LiteSVM,
//     user_stake: &Pubkey,
// ) -> UserStake {
//     let account = svm
//         .get_account(user_stake)
//         .expect("User stake account not found");

//     let mut data: &[u8] = &account.data;

//     UserStake::try_deserialize(&mut data)
//         .expect("Failed to deserialize user stake account")
// }