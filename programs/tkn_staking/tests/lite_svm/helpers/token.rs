use {
    crate::helpers::setup::send_transaction, anchor_lang::{prelude::{pubkey, Pubkey}, solana_program::{program_pack::Pack, system_instruction}}, anchor_spl::token::spl_token, litesvm::LiteSVM, solana_keypair::Keypair, solana_message::{Message, VersionedMessage}, solana_signer::Signer, solana_transaction::versioned::VersionedTransaction
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

    let initialize_token_ix = spl_token::instruction::initialize_account3(&spl_token::ID, &token_account.pubkey(), mint.pubkey(), owner).unwrap();

    send_transaction(svm, payer, &[create_ix, initialize_token_ix], &[payer, token_account]);
}


