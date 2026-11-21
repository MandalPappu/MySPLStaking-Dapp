use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    anchor_spl::token,
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

use anchor_lang::{prelude::system_instruction, Key};
use anchor_spl::token::{spl_token, Mint};
use tkn_staking::{accounts, constants::POOL_SEED, instruction, ID as PROGRAM_ID, states::StakingPool};

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

fn send_transaction(
    svm: &mut LiteSVM,
    payer: &Keypair,
    signers: &[&Keypair],
    instructions: Vec<Instruction>,
) {
    let mut message = Message::new(&instructions, Some(&payer.pubkey()));

    message.recent_blockhash = svm.latest_blockhash();
    
    let versioned_message = VersionedMessage::Legacy(message);

    let transaction = VersionedTransaction::try_new(versioned_message, signers).unwrap();

    let result = svm.send_transaction(transaction);

    assert!(result.is_ok(), "transaction failed: {:?}", result.err());
}

fn create_mint(svm: &mut LiteSVM, payer: &Keypair, mint: &Keypair, decimals: u8) {
    let rent = svm.minimum_balance_for_rent_exemption(Mint::LEN);

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let initialize_mint_ix = spl_token::instruction::initialize_mint(
        &token::ID,
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )
    .unwrap();

    let mut message = Message::new(
        &[create_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
    );
    message.recent_blockhash = svm.latest_blockhash();

    let versioned_message = VersionedMessage::Legacy(message);

    let transaction = VersionedTransaction::try_new(versioned_message, &[payer, mint]).unwrap();

    svm.send_transaction(transaction).unwrap();
}

#[test]
fn test_initialize() {
    // 1. Initialize LiteSVM
    let mut svm = LiteSVM::new();

    // Load compiled program
    let program_bytes = include_bytes!("../../../../target/deploy/tkn_staking.so");
    

    svm.add_program(PROGRAM_ID, program_bytes).unwrap();

    // 2. Create authority
    let authority = Keypair::new();

    svm.airdrop(&authority.pubkey(), 5 * LAMPORTS_PER_SOL)
        .unwrap();

    let staking_mint = Keypair::new();
    let reward_mint = Keypair::new();

    // 3. initializing two spl token 
    create_mint(&mut svm, &authority, &staking_mint, 6);
    create_mint(&mut svm, &authority, &reward_mint, 6);

    // 4. derive Pool PDA;
    let (pool, _) = Pubkey::find_program_address(
        &[POOL_SEED, staking_mint.pubkey().as_ref()], &PROGRAM_ID);
    
    let (token_authority, _) = Pubkey::find_program_address(
        &[b"token_authority", pool.key().as_ref()], &PROGRAM_ID);

    // 5. Initialize pool instruction

    let initialize_pool_ix = Instruction {
        program_id:PROGRAM_ID,
        accounts: accounts::InitializePool {
            authority: authority.pubkey(),
            staking_mint:staking_mint.pubkey(),
            reward_mint:reward_mint.pubkey(),
            pool,
            token_authority,
            token_program:token::ID,
            system_program:system_program::ID,
        }.to_account_metas(None),
        data: instruction::InitializePool {
            reward_rate:1_000,
        }.data(),
    };   

    send_transaction(&mut svm, &authority, &[&authority], vec![initialize_pool_ix]);

    // 6. verify Pool

    let pool_account = svm.get_account(&pool).expect("pool account not found!");

    let pool_state = StakingPool::try_deserialize(&mut pool_account.data.as_slice()).unwrap();

    assert_eq!(pool_state.authority, authority.pubkey());

    assert_eq!(pool_state.staking_mint, staking_mint.pubkey());

    assert_eq!(pool_state.reward_mint, reward_mint.pubkey());

    assert_eq!(pool_state.reward_rate, 1_000);

}
