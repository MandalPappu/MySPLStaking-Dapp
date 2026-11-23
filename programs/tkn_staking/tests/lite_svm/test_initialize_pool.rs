use {
    crate::helpers::{
        accounts::{
            derive_pool_pda,
            derive_token_authority_pda,
            get_pool_state
        },
        instructions::initialize_pool_instruction,
        setup::{send_transaction, setup_svm},
        token::create_mint
    },
    anchor_lang::prelude::Pubkey,
    solana_keypair::Keypair,
    solana_signer::Signer
};

// =====================================================
// TEST 1: INITIALIZE POOL SUCCESS
// =====================================================

#[test]
fn test_initialize_pool_success() {
    let (mut svm, payer) = setup_svm();

    let staking_mint = Keypair::new();
    let reward_mint = Keypair::new();

    create_mint(&mut svm, &payer, &staking_mint, 9);
    create_mint(&mut svm, &payer, &reward_mint, 9);

    let (pool_pda, pool_bump) = derive_pool_pda(&staking_mint.pubkey());
    let (token_authority_pda, _) = derive_token_authority_pda(&pool_pda);

    let reward_rate:u64 = 1_000;

    let initialize_pool_ix = initialize_pool_instruction(payer.pubkey(), staking_mint.pubkey(), reward_mint.pubkey(), pool_pda, token_authority_pda, reward_rate);

    send_transaction(&mut svm, &payer, &[initialize_pool_ix], &[&payer]);

    let pool_state = get_pool_state(&mut svm, &pool_pda);

    assert_eq!(pool_state.authority, payer.pubkey(), "incorrect pool authoriyt");
    assert_eq!(pool_state.staking_mint, staking_mint.pubkey(), "incorrect staking mint");
    assert_eq!(pool_state.reward_mint, reward_mint.pubkey(), "incorrect reward mint");
    assert_eq!(pool_state.reward_rate, reward_rate, "Incorrect reward rate");

    assert_eq!(pool_state.total_staked, 0, "Initial total staked should be zero");
     assert_eq!(pool_state.reward_per_token_stored , 0, "Initial reward budget should be zero");

    assert_eq!(pool_state.staking_vault, Pubkey::default(), "Staking vault should not be initialized yet");

    assert_eq!(pool_state.reward_vault, Pubkey::default(), "Reward vault should not be initialized yet");
    assert_eq!(pool_state.bump, pool_bump, "Incorrect pool PDA bump");


    println!("Initialize pool success test passed!");

}