use std::env;

use borsh::{BorshDeserialize, BorshSerialize};
use custom_solana_programs::hello_name_program;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::signature::{Keypair, read_keypair_file};
use solana_sdk::signer::Signer;

use crate::transactions::hello_name_program::call_hello_name_transaction;

mod transactions;

fn main() {
    let root_wallet_path = &env::args()
        .by_ref()
        .collect::<Vec<String>>()[1];
    let rpc_client = RpcClient::new("http://localhost:8899");
    let root_account = load_wallet_from_local(&rpc_client, root_wallet_path);

    run_hello_name_instruction(&rpc_client, &root_account.keypair);

    return;
}

struct RootAccount {
    keypair: Keypair,
    account: Account,
}

fn run_hello_name_instruction(rpc_client: &RpcClient, root_account: &Keypair) {
    let instruction_data = hello_name_program::NamesData {
        group_leader: String::from("KeyApp"),
        group_members: vec!["Chingiz", "Nick", "Tengiz", "Kostya", "Davran", "Eduard"]
            .iter()
            .map(|&x| String::from(x))
            .collect(),
    };
    call_hello_name_transaction(&instruction_data, root_account, rpc_client);
}

fn load_wallet_from_local(rpc_client: &RpcClient, path: &str) -> RootAccount {
    let wallet = read_keypair_file(path)
        .expect("Example requires a keypair file");
    let pub_key = wallet.pubkey();

    let account = rpc_client.get_account(&pub_key).unwrap();
    println!("{}", from_lamports(account.lamports, 9));
    println!("{}", lamports_to_sol(account.lamports));
    println!("{}", account.owner);
    println!("{}", &pub_key);

    RootAccount { keypair: wallet, account }
}

fn from_lamports(lamports: u64, decimals: u32) -> f64 {
    lamports as f64 / (10_u64.pow(decimals)) as f64
}
