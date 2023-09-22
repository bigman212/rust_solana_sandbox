use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenAccountErrorType {
    #[error("keypair failed to read from IO")]
    MintAccountKeypairReadFailure(
        #[from] Box<dyn std::error::Error>
    ),
    #[error("rpc client returned error `{0}`")]
    RpcClientError(
        #[from] solana_client::client_error::ClientError
    ),
    #[error("init instruction failed: {:?}", 0)]
    InitInstructionError(
        #[from] solana_sdk::program_error::ProgramError
    ),
}