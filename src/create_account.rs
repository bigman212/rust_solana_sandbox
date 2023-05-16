pub mod create_account_utils {
    use std::fs::{DirBuilder};
    use std::path::Path;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{system_instruction, system_program};
    use solana_sdk::account::Account;
    use solana_sdk::signature::{Keypair, write_keypair_file};
    use solana_sdk::signer::Signer;
    use solana_sdk::transaction::Transaction;

    fn get_wallets_dir() -> String {
        String::from("wallets/")
    }

    pub fn create_new_account(
        rpc_client: &RpcClient,
        payer: &Keypair,
    ) -> solana_client::client_error::Result<Account> {
        let new_key_pair = Keypair::new();
        println!("new public key generated {}", &new_key_pair.pubkey());
        save_new_keypair_to_file(&new_key_pair);

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

        println!("Account creation success: {}", _sig);

        rpc_client.get_account(&new_key_pair.pubkey())
    }

    fn save_new_keypair_to_file(keypair: &Keypair) {
        check_wallets_dir();

        let dir = get_wallets_dir();
        let wallet_path = Path::new(&dir)
            .join(format!("{}.json", keypair.pubkey()));

        write_keypair_file(&keypair, wallet_path)
            .unwrap();
    }

    fn check_wallets_dir() {
        let wallets_dir = get_wallets_dir();
        let wallets_path = Path::new(&wallets_dir);

        if !wallets_path.exists() {
            // create_dir_all("").unwrap();
            DirBuilder::new()
                .create(wallets_path)
                .expect("No way to create a dir");
        }
    }
}