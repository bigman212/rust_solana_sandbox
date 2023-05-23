use std::path::Path;
use std::str::FromStr;
use std::string::ToString;

use mpl_token_metadata::solana_program::instruction::Instruction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash,
    program_error::ProgramError,
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::solana_program::pubkey::Pubkey;
use spl_token::{
    solana_program::program_pack::Pack,
    state::Mint,
};

use crate::transactions::tkeyapp::error::TokenAccountErrorType;

pub mod tkeyapp_token_metadata {
    use solana_sdk::pubkey;
    use solana_sdk::pubkey::Pubkey;

    pub const NAME: &str = "TKeyApp";
    pub const SYMBOL: &str = "tkeyapp";
    pub const MINT: Pubkey = pubkey!("4uZjiUSrRwygmoLBR6hN8wcmYq59Wsw5ugcHKKxbd1Lz");
}

pub fn create_new_token_mint_account(
    rpc_client: RpcClient,
    payer: Keypair,
    mint_account_key_payer: Keypair,
) -> [Instruction; 2] {
    // This account stores general information about the token and who has permissions over it
    let mint_exemption = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN)
        .expect("rent exemption was not fetched");
    let create_mint_account_instruction =
        system_instruction::create_account(
            &payer.pubkey(),
            &mint_account_key_payer.pubkey(),
            mint_exemption,
            Mint::LEN as u64,
            &spl_token::ID,
        );
    let init_mint_instruction =
        spl_token::instruction::initialize_mint(
            &spl_token::ID,
            &mint_account_key_payer.pubkey(),
            &payer.pubkey(),
            None,
            9,
        ).expect("initialize_mint was not created");

    [create_mint_account_instruction, init_mint_instruction]
}

/// Doesn't work
pub fn create_metadata_account_instruction(
    token_mint_account: Keypair,
    payer: Keypair,
) -> Instruction {
    let pda_program_address = mpl_token_metadata::pda::find_metadata_account(
        &token_mint_account.pubkey()
    ).0;

    mpl_token_metadata::instruction::create_metadata_accounts_v3(
        Pubkey::from_str("HeNqBTQSGSpvGoAHWFk9VGL8HmSidBKBrKtYPCrcDB3G").unwrap(),
        pda_program_address,
        token_mint_account.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        payer.pubkey(),
        tkeyapp_token_metadata::NAME.to_string(),
        tkeyapp_token_metadata::SYMBOL.to_string(),
        "http://commondatastorage.googleapis.com/codeskulptor-assets/lathrop/asteroid_blend.png".to_string(),
        None,
        0,
        false,
        true,
        None,
        None,
        None,
    )
}

pub fn create_token_account_transaction(
    new_token_account_owner: Keypair,
    payer: Keypair,
    recent_blockhash: Hash,
) -> Result<Transaction, TokenAccountErrorType> {
    let token_keypair_path = Path::new("./wallets/tkeyapp_4uZjiUSrRwygmoLBR6hN8wcmYq59Wsw5ugcHKKxbd1Lz");
    let mint_account_key_pair = read_keypair_file(token_keypair_path)?;

    let spl_program_id = &spl_token::ID;

    println!("Mint account public key: {}", mint_account_key_pair.pubkey());

    let ata_address = spl_associated_token_account::get_associated_token_address(
        &new_token_account_owner.pubkey(),
        &mint_account_key_pair.pubkey(),
    );
    println!("ATA address: {ata_address}");

    let init_ata_instruction =
        spl_associated_token_account::instruction::create_associated_token_account(
            &payer.pubkey(),
            &new_token_account_owner.pubkey(),
            &mint_account_key_pair.pubkey(),
            spl_program_id,
        );

    Ok(
        Transaction::new_signed_with_payer(
            &[init_ata_instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        )
    )
}

pub fn mint_token_account(
    payer: Keypair,
    mint_token_keypair: Keypair,
    associated_token_account_address: &Pubkey,
    associated_token_account_owner: Keypair,
    recent_blockhash: Hash,
    amount: u64,
) -> Result<Transaction, ProgramError> {
    let mint_token_address = &mint_token_keypair.pubkey();

    let mint_tokens_instruction = spl_token::instruction::mint_to_checked(
        &spl_token::ID,
        mint_token_address,
        associated_token_account_address,
        &associated_token_account_owner.pubkey(),
        &[&payer.pubkey(), &mint_token_keypair.pubkey()],
        amount.pow(9),
        9,
    )?;

    return Ok(Transaction::new_signed_with_payer(
        &[mint_tokens_instruction],
        Some(&payer.pubkey()),
        &[&payer, &associated_token_account_owner],
        recent_blockhash,
    ));
}