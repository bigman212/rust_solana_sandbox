#[derive(Debug)]
pub enum TokenAccountErrorType {
    MintAccountKeypairReadFailure(Box<dyn std::error::Error>),
    RpcClientError(solana_client::client_error::ClientError),
    InitInstructionError(solana_sdk::program_error::ProgramError),
}

impl From<Box<dyn std::error::Error>> for TokenAccountErrorType {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        TokenAccountErrorType::MintAccountKeypairReadFailure(value)
    }
}

impl From<solana_client::client_error::ClientError> for TokenAccountErrorType {
    fn from(value: solana_client::client_error::ClientError) -> Self {
        TokenAccountErrorType::RpcClientError(value)
    }
}

impl From<solana_sdk::program_error::ProgramError> for TokenAccountErrorType {
    fn from(value: solana_sdk::program_error::ProgramError) -> Self {
        TokenAccountErrorType::InitInstructionError(value)
    }
}