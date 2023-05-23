use std::fs::DirBuilder;
use std::path::Path;
use std::str::FromStr;

use borsh::BorshSerialize;
use custom_solana_programs::hello_name_program;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, write_keypair_file},
    signer::Signer,
    system_instruction,
    system_program,
    transaction::Transaction,
};

use crate::transactions::utils;

pub fn call_create_new_account(
    rpc_client: &RpcClient,
    payer: &Keypair,
) -> solana_client::client_error::Result<Account> {
    let new_key_pair = Keypair::new();
    println!("new public key generated {}", new_key_pair.pubkey());

    utils::save_new_keypair_to_file(&new_key_pair, None);

    let rent = rpc_client.get_minimum_balance_for_rent_exemption(0)
        .unwrap();

    let instruction = system_instruction::create_account(
        &payer.pubkey(),
        &new_key_pair.pubkey(),
        rent,
        0,
        &system_program::ID,
    );

    let blockhash = rpc_client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, &new_key_pair],
        blockhash,
    );

    let _sig = rpc_client.send_and_confirm_transaction(&tx)
        .unwrap();

    println!("Account creation success: {_sig}");

    rpc_client.get_account(&new_key_pair.pubkey())
}
