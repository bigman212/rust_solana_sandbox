use std::fs::DirBuilder;
use std::path::Path;
use solana_sdk::signature::{Keypair, Signer, write_keypair_file};

pub fn get_wallets_dir() -> String {
    String::from("wallets/")
}

pub fn save_new_keypair_to_file(keypair: &Keypair) {
    check_wallets_dir();

    let dir = get_wallets_dir();
    let wallet_path = Path::new(&dir)
        .join(format!("{}.json", keypair.pubkey()));

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