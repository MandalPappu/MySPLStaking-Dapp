use {
    crate::helpers::{
        accounts::{
            derive_pool_PDA,
            derive_token_authority_PDA,
            get_pool_state
        },
        instructions::{initialize_pool_instruction, initialize_vaults_instruction},
        setup::{send_transaction, setup_svm},
        token::{create_mint, get_token_balance}
    }, anchor_lang::prelude::{program_pack::Pack}, anchor_spl::token::spl_token, solana_keypair::Keypair, solana_signer::Signer
};

#[test]
fn test_initialize_vault_success() {
    let (mut svm, payer) = setup_svm();

       // Create staking and reward mints.
    let staking_mint = Keypair::new();
    let reward_mint = Keypair::new();

    create_mint(&mut svm, &payer, &staking_mint, 9);
    create_mint(&mut svm, &payer, &reward_mint, 9);

    // Derive staking pool PDA.
    let (pool, _) = derive_pool_PDA(&staking_mint.pubkey());
    let (token_authority_pda, _) = derive_token_authority_PDA(&pool);

    let reward_rate:u64 = 1_000;

    let initialize_vault_ix = initialize_pool_instruction(payer.pubkey(), staking_mint.pubkey(), reward_mint.pubkey(), pool, token_authority_pda, reward_rate);

    send_transaction(&mut svm, &payer, &[initialize_vault_ix], &[&payer]);


    // Generate token vault account keypairs.
    let staking_vault = Keypair::new();
    let reward_vault = Keypair::new();

    let initialize_vault_ix = initialize_vaults_instruction(payer.pubkey(), pool, staking_mint.pubkey(), reward_mint.pubkey(), staking_vault.pubkey(), reward_vault.pubkey());

    send_transaction(&mut svm, &payer, &[initialize_vault_ix], &[&payer, &staking_vault, &reward_vault]);

    let pool_state = get_pool_state(&mut svm, &pool);


    assert_eq!(pool_state.staking_vault, staking_vault.pubkey());
    assert_eq!(pool_state.reward_vault, reward_vault.pubkey());

    let staking_vault_account = svm.get_account(&staking_vault.pubkey()).expect("staking vault account not found!");

    let staking_token_account =spl_token::state::Account::unpack(&staking_vault_account.data).expect("staking vault account failed to deserialize!");

    assert_eq!(staking_token_account.mint, staking_mint.pubkey());
    assert_eq!(staking_token_account.owner, pool);

    let reward_vault_account = svm.get_account(&reward_vault.pubkey()).expect("failed to find reward vault account!");

    let reward_token_account = spl_token::state::Account::unpack(&reward_vault_account.data).expect("failed to deserialize reward vault account");

    assert_eq!(reward_token_account.mint, reward_mint.pubkey());
    assert_eq!(reward_token_account.owner, pool);

    assert_eq!(get_token_balance(&mut svm, &staking_vault.pubkey()), 0);
    assert_eq!(get_token_balance(&mut svm, &reward_vault.pubkey()), 0);
}