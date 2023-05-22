use std::env;

use custom_solana_programs::hello_name_program;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::signature::{Keypair, read_keypair_file};
use solana_sdk::signer::Signer;

use crate::transactions::create_account::custom_transactions::call_hello_name;

mod transactions;

fn main() {
    let root_wallet_path: Vec<String> = env::args().collect();
    let instruction_data = hello_name_program::NamesData {
        group_leader: String::from("KeyApp"),
        group_members: vec!["Chingiz", "Nick", "Tengiz", "Kostya", "Davran", "Eduard"]
            .iter()
            .map(|&x| String::from(x))
            .collect(),
    };
    let rpc_client = RpcClient::new("http://localhost:8899");
    let root_account = load_wallet_from_local(&rpc_client, &root_wallet_path[1]);

    call_hello_name(&instruction_data, &root_account.keypair, rpc_client);

    // custom_transactions::create_new_account(&rpc_client, &root_account.keypair)
    //     .expect("Expected ok");

    return;
}

struct RootAccount {
    keypair: Keypair,
    account: Account,
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

    RootAccount {
        keypair: wallet,
        account,
    }
}

fn from_lamports(lamports: u64, decimals: u32) -> f64 {
    lamports as f64 / (10_u64.pow(decimals)) as f64
}
