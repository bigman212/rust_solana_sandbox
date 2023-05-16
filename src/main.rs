mod create_account;

use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
use solana_sdk::native_token::{lamports_to_sol};
use solana_sdk::signature::{Keypair, read_keypair_file};
use solana_sdk::signer::Signer;
use crate::create_account::create_account_utils;

fn main() {
    let rpc_client = RpcClient::new("http://localhost:8899");
    let root_account = load_wallet_from_local(&rpc_client);
    create_account_utils::create_new_account(&rpc_client, &root_account.keypair)
        .expect("Expected ok");

    return;
}

struct RootAccount {
    keypair: Keypair,
    account: Account,
}

fn load_wallet_from_local(rpc_client: &RpcClient) -> RootAccount {
    let wallet = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
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
