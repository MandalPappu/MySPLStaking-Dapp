use {
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_signer::Signer,
    solana_message::{Message, VersionedMessage},
    solana_transaction::versioned::VersionedTransaction,
    anchor_lang::solana_program::instruction::Instruction,
    tkn_staking::ID as PROGRAM_ID
};


pub fn setup_svm() -> (LiteSVM, Keypair) {

    let mut svm = LiteSVM::new()
            .with_default_programs();
    svm.add_program_from_file(PROGRAM_ID, "target/deploy/tkn_staking.so").expect("failed to load tkn_staking program!");

    let payer = Keypair::new();

    svm.airdrop(&payer.pubkey(), 2 * 1_000_000_000).expect("airdrop failed!");

    (svm, payer)
}

pub fn send_transaction(
    svm: &mut LiteSVM,
    payer: &Keypair,
    instructions:&[Instruction],
    signers:&[&Keypair]
) {
    let message = Message::new_with_blockhash(instructions, Some(&payer.pubkey()), &svm.latest_blockhash());

    let tx = VersionedTransaction::try_new(
        VersionedMessage::Legacy(message), signers).expect("transaction creation failed!");
    
    svm.send_transaction(tx).expect("transaction excution failed!");
}