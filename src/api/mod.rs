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

pub mod albums;
pub mod auth;
pub mod client;
pub mod pager;

pub use client::SpotifyClient;

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
