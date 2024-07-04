use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum IndexerError {
    IndexerCustom(String),
    IndexerDBError(sqlx::Error),
    IndexerIOError(std::io::Error),
    IndexerTomlDeError(toml::de::Error),
    IndexerHexError(rustc_hex::FromHexError),
    IndexerParseUrlError(url::ParseError),
    IndexerRedisError(redis::RedisError),
}

impl From<redis::RedisError> for IndexerError {
    fn from(e: redis::RedisError) -> Self {
        IndexerError::IndexerRedisError(e)
    }
}

impl From<String> for IndexerError {
    fn from(e: String) -> Self {
        IndexerError::IndexerCustom(e)
    }
}

impl From<url::ParseError> for IndexerError {
    fn from(e: url::ParseError) -> Self {
        IndexerError::IndexerParseUrlError(e)
    }
}

impl From<rustc_hex::FromHexError> for IndexerError {
    fn from(e: rustc_hex::FromHexError) -> Self {
        IndexerError::IndexerHexError(e)
    }
}

impl From<std::io::Error> for IndexerError {
    fn from(e: std::io::Error) -> Self {
        IndexerError::IndexerIOError(e)
    }
}

impl From<toml::de::Error> for IndexerError {
    fn from(e: toml::de::Error) -> Self {
        IndexerError::IndexerTomlDeError(e)
    }
}

impl From<sqlx::Error> for IndexerError {
    fn from(e: sqlx::Error) -> Self {
        IndexerError::IndexerDBError(e)
    }
}

pub type Result<T> = core::result::Result<T, IndexerError>;

impl IntoResponse for IndexerError {
    fn into_response(self) -> Response {
        let err_msg = match self {
            IndexerError::IndexerCustom(e) => e,
            IndexerError::IndexerDBError(e) => e.to_string(),
            IndexerError::IndexerIOError(e) => e.to_string(),
            IndexerError::IndexerTomlDeError(e) => e.to_string(),
            IndexerError::IndexerHexError(e) => e.to_string(),
            IndexerError::IndexerParseUrlError(e) => e.to_string(),
            IndexerError::IndexerRedisError(e) => e.to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, err_msg).into_response()
    }
}
