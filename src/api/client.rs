use futures::future::BoxFuture;
use http_client::HttpClient;

use crate::oauth::TokenCache;

pub type HttpClientResult = Result<http_client::Response, http_client::Error>;
pub type HttpClientFuture = BoxFuture<'static, HttpClientResult>;

/// Marker trait to indicate operations that need a client capable of handling Spotify client
/// client session details. Specifically, this client is responsible for setting authorization
/// headers, retry after cooldown, etc.
pub trait SpotifyClient: HttpClient {
    fn send_authorized(&self, req: http_client::Request) -> HttpClientFuture;
}

/// HTTP client for interactions with Spotify. Handles authorization, API timeout, etc.
#[derive(Debug)]
pub struct BasicSpotifyClient<C, T> {
    client: C,
    token_cache: T,
}

impl<C: HttpClient, T: TokenCache> BasicSpotifyClient<C, T> {
    pub fn new(client: C, token_cache: T) -> Self {
        BasicSpotifyClient {
            client,
            token_cache,
        }
    }
}

impl<C, T> HttpClient for BasicSpotifyClient<C, T>
where
    C: HttpClient,
    T: 'static + TokenCache + Send + Sync + Unpin,
{
    fn send(&self, req: http_client::Request) -> HttpClientFuture {
        self.client.send(req)
    }
}

impl<C, T> SpotifyClient for BasicSpotifyClient<C, T>
where
    C: HttpClient,
    T: 'static + TokenCache + Send + Sync + Unpin,
{
    fn send_authorized(&self, _req: http_client::Request) -> HttpClientFuture {
        todo!()
    }
}
