use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum IndexerError {
    Custom(String),
    DBError(sqlx::Error),
    IOError(std::io::Error),
    TomlDeError(toml::de::Error),
    HexError(rustc_hex::FromHexError),
    ParseUrlError(url::ParseError),
}

impl From<String> for IndexerError {
    fn from(e: String) -> Self {
        IndexerError::Custom(e)
    }
}

impl From<url::ParseError> for IndexerError {
    fn from(e: url::ParseError) -> Self {
        IndexerError::ParseUrlError(e)
    }
}

impl From<rustc_hex::FromHexError> for IndexerError {
    fn from(e: rustc_hex::FromHexError) -> Self {
        IndexerError::HexError(e)
    }
}

impl From<std::io::Error> for IndexerError {
    fn from(e: std::io::Error) -> Self {
        IndexerError::IOError(e)
    }
}

impl From<toml::de::Error> for IndexerError {
    fn from(e: toml::de::Error) -> Self {
        IndexerError::TomlDeError(e)
    }
}

impl From<sqlx::Error> for IndexerError {
    fn from(e: sqlx::Error) -> Self {
        IndexerError::DBError(e)
    }
}

pub type Result<T> = core::result::Result<T, IndexerError>;

impl IntoResponse for IndexerError {
    fn into_response(self) -> Response {
        let err_msg = match self {
            IndexerError::Custom(e) => e,
            IndexerError::DBError(e) => e.to_string(),
            IndexerError::IOError(e) => e.to_string(),
            IndexerError::TomlDeError(e) => e.to_string(),
            IndexerError::HexError(e) => e.to_string(),
            IndexerError::ParseUrlError(e) => e.to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response()
    }
}
