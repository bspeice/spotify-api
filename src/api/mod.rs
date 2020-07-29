use thiserror::Error;

// Set a URL parameter using the parameter name as its value.
//
// ```rust
// use http_types::Url;
// let mut url = Url::parse("https://google.com/").unwrap();
// let param = "my_value";
// set_query_param!(url, param);
// assert_eq!(url.as_str(), "https://google.com/?param=my_value")
// ```
macro_rules! set_query_param {
    ($url:expr, $param:ident) => {
        if let Some($param) = $param {
            $url.set_query(Some(&format!(concat!(stringify!($param), "={}"), $param)))
        }
    };
}

macro_rules! set_query_param_joined {
    ($url:expr, $param:ident) => {
        if !$param.is_empty() {
            let value = $param.join(",");
            $url.set_query(Some(&format!(concat!(stringify!($param), "={}"), value)))
        }
    };
}

pub mod albums;
pub mod artists;
pub mod auth;
pub mod browse;
pub mod client;
pub mod pager;

pub use client::{ClientExt, SpotifyClient};

#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    Json(#[from] serde_json::Error),

    #[error("")]
    Http(http_client::Error),

    #[error("client was asked to send an authorized request, but no token was available")]
    MissingToken,

    #[error("")]
    Url(#[from] http_types::url::ParseError),
}

impl From<http_client::Error> for Error {
    fn from(e: http_client::Error) -> Self {
        Self::Http(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
