use crate::helpers::setup::send_transaction;
use {
    anchor_lang::{prelude::Pubkey, solana_program::{program_pack::Pack, system_instruction}}, anchor_spl::token::spl_token, litesvm::LiteSVM, solana_keypair::Keypair, solana_signer::Signer
};


// =====================================================
// 1. CREATE SPL TOKEN MINT
// =====================================================

pub fn create_mint(
    svm: &mut LiteSVM,
    payer:&Keypair,
    mint:&Keypair,
    decimals:u8
) {
    let mint_size = spl_token::state::Mint::LEN;

    let rent = svm.minimum_balance_for_rent_exemption(mint_size);

    // create mint account
    let create_acccount_ix = system_instruction::create_account(&payer.pubkey(), &mint.pubkey(), rent, mint_size as u64, &spl_token::ID);

    let initialize_mint_ix = spl_token::instruction::initialize_mint(&spl_token::ID, &mint.pubkey(), &payer.pubkey(), None, decimals).unwrap();

    send_transaction(svm, payer, &[create_acccount_ix, initialize_mint_ix], &[payer, mint]);
}

// =====================================================
// 2. CREATE token account
// =====================================================


pub fn create_token_account(
    svm: &mut LiteSVM,
    token_account:&Keypair,
    mint:&Pubkey,
    payer:&Keypair,
    owner:&Pubkey
){
    let account_size = spl_token::state::Account::LEN;
    let rent = svm.minimum_balance_for_rent_exemption(account_size);

    // create token account 
    let create_ix = system_instruction::create_account(&payer.pubkey(), &token_account.pubkey(), rent, account_size as u64, &spl_token::ID);

    // initialize token account

    let initialize_token_ix = spl_token::instruction::initialize_account3(&spl_token::ID, &token_account.pubkey(), mint, owner).unwrap();

    send_transaction(svm, payer, &[create_ix, initialize_token_ix], &[payer, token_account]);
}

// =====================================================
// 3. MINT TOKENS
// =====================================================

pub fn mint_tokens(
    svm: &mut LiteSVM,
    payer:&Keypair,
    mint:&Pubkey,
    destination:&Pubkey,
    amount:u64,
    mint_authority:&Keypair
) {
    let mint_to_ix = spl_token::instruction::mint_to(&spl_token::ID, mint, destination, &mint_authority.pubkey(), &[], amount).unwrap();

    send_transaction(svm, payer, &[mint_to_ix], &[payer, mint_authority]);
}


// =====================================================
// 4. GET TOKEN BALANCE
// =====================================================

pub fn get_token_balance(
    svm: &mut LiteSVM,
    token_account:&Pubkey
) -> u64 {
    let account = svm.get_account(token_account).expect("token account not found!");

    let token_state = spl_token::state::Account::unpack(&account.data).expect("Failed to deserialize token account");

    token_state.amount
}


// =====================================================
// 5. GET MINT SUPPLY
// =====================================================

pub fn get_mint_supply(svm: &mut LiteSVM, mint:&Pubkey) -> u64 {
    let mint_account = svm.get_account(mint).expect("mint account not found!");

    let mint_state = spl_token::state::Mint::unpack(&mint_account.data).expect("Failed to deserialize mint");

    mint_state.supply
}



