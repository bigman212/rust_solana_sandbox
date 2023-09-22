use std::env;
use std::path::Path;

use custom_solana_programs::hello_name_program;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::read_keypair_file;

use crate::transactions::hello_name_program::call_hello_name_transaction;

mod transactions;

fn main() {
    let root_wallet_path = &env::args()
        .nth(1)
        .expect("Expected passing root wallect path json in program args");
    let rpc_client = RpcClient::new("http://localhost:8899");
    let root_account = read_keypair_file(Path::new(root_wallet_path))
        .unwrap();

    return;
}

struct RootAccount {
    keypair: Keypair,
    account: Account,
}

fn run_hello_name_instruction(rpc_client: &RpcClient, root_account: &Keypair) {
    let instruction_data = hello_name_program::NamesData {
        group_leader: String::from("KeyApp"),
        group_members: vec![
            "Chingiz".to_string(),
            "Nick".to_string(),
            "Tengiz".to_string(),
            "Kostya".to_string(),
            "Davran".to_string(),
            "Eduard".to_string()
        ]
    };
    call_hello_name_transaction(&instruction_data, root_account, rpc_client);
}


