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

pub fn call_hello_name_transaction(
    data: &hello_name_program::NamesData,
    payer: &Keypair,
    rpc_client: &RpcClient,
) {
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let instruction = Instruction {
        program_id: hello_name_program::PROGRAM_ID,
        data: data.try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            }
        ],
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    println!("{}", rpc_client.send_and_confirm_transaction(&tx).unwrap());
}