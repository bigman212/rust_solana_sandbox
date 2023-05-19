use std::fs::DirBuilder;
use std::path::Path;

use solana_client::client_error::{ClientError, ClientErrorKind};
use solana_client::rpc_request::RpcError::RpcResponseError;
use solana_client::rpc_request::RpcResponseErrorData;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::signature::{Keypair, read_keypair_file, Signer, write_keypair_file};

use crate::RootAccount;

pub fn get_wallets_dir() -> String {
    String::from("wallets/")
}

pub fn save_new_keypair_to_file(keypair: &Keypair, file_name: Option<String>) {
    let file_name = file_name.map_or(
        format!("{}.json", keypair.pubkey()),
        |it| format!("{}_{}", it, keypair.pubkey()),
    );

    check_wallets_dir();

    let dir = get_wallets_dir();
    let wallet_path = Path::new(&dir).join(file_name);

    write_keypair_file(&keypair, wallet_path)
        .expect("Failed to write_keypair_file");
}

fn check_wallets_dir() {
    let wallets_dir = get_wallets_dir();
    let wallets_path = Path::new(&wallets_dir);

    if !wallets_path.exists() {
        DirBuilder::new()
            .create(wallets_path)
            .expect("No way to create a dir");
    }
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

pub fn extract_error_logs(error: ClientError) -> String {
    let error: ClientErrorKind = ClientError::from(error).kind;
    let empty_string = String::new();

    if let ClientErrorKind::RpcError(it) = error {
        if let RpcResponseError { code: _code, message: _message, data } = it {
            if let RpcResponseErrorData::SendTransactionPreflightFailure(result) = data {
                result.logs.unwrap()
                    .clone()
                    .join("\n")
            } else {
                empty_string
            }
        } else {
            empty_string
        }
    } else {
        empty_string
    }
}