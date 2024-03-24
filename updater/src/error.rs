use ethers::contract::ContractError;

use ethers::providers::ProviderError;
use ethers::providers::{Http, Provider};

#[derive(Debug)]
pub enum UpdaterError {
    Custom(String),
    DBError(sqlx::Error),
    JoinError(tokio::task::JoinError),
    ParseUrlError(url::ParseError),
    HexError(rustc_hex::FromHexError),
    TomlDeError(toml::de::Error),
    IOError(std::io::Error),
    EthersContractError(ContractError<Provider<Http>>),
    EthersProviderError(ProviderError),
}

impl From<ProviderError> for UpdaterError {
    fn from(e: ProviderError) -> Self {
        UpdaterError::EthersProviderError(e)
    }
}

impl From<ContractError<Provider<Http>>> for UpdaterError {
    fn from(e: ContractError<Provider<Http>>) -> Self {
        UpdaterError::EthersContractError(e)
    }
}

impl From<String> for UpdaterError {
    fn from(e: String) -> Self {
        UpdaterError::Custom(e)
    }
}

impl From<url::ParseError> for UpdaterError {
    fn from(e: url::ParseError) -> Self {
        UpdaterError::ParseUrlError(e)
    }
}

impl From<sqlx::Error> for UpdaterError {
    fn from(e: sqlx::Error) -> Self {
        UpdaterError::DBError(e)
    }
}

impl From<tokio::task::JoinError> for UpdaterError {
    fn from(e: tokio::task::JoinError) -> Self {
        UpdaterError::JoinError(e)
    }
}

impl From<rustc_hex::FromHexError> for UpdaterError {
    fn from(e: rustc_hex::FromHexError) -> Self {
        UpdaterError::HexError(e)
    }
}

impl From<toml::de::Error> for UpdaterError {
    fn from(e: toml::de::Error) -> Self {
        UpdaterError::TomlDeError(e)
    }
}

impl From<std::io::Error> for UpdaterError {
    fn from(e: std::io::Error) -> Self {
        UpdaterError::IOError(e)
    }
}

pub type Result<T> = core::result::Result<T, UpdaterError>;
