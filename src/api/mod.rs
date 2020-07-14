use thiserror::Error;

pub mod auth;
pub mod client;

#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    Json(#[from] serde_json::Error),

    #[error("")]
    Http(http_client::Error),

    #[error("")]
    Url(#[from] http_types::url::ParseError),
}

impl From<http_client::Error> for Error {
    fn from(e: http_client::Error) -> Self {
        Self::Http(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
