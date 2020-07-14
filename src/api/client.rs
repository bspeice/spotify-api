use http_client::HttpClient;
use std::future::Future;
use std::pin::Pin;

use crate::oauth::TokenCache;

/// HTTP client for interactions with Spotify. Handles authorization, API timeout, etc.
#[derive(Debug)]
pub struct SpotifyClient<C, T> {
    client: C,
    token_cache: T,
}

impl<C: HttpClient, T: TokenCache> SpotifyClient<C, T> {
    pub fn new(client: C, token_cache: T) -> Self {
        SpotifyClient {
            client,
            token_cache,
        }
    }
}

impl<C, T> HttpClient for SpotifyClient<C, T>
where 
    C: HttpClient,
    T: 'static + TokenCache + Send + Sync + Unpin 
{
    fn send(
        &self,
        _req: http_client::Request,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<http_client::Response, http_client::Error>> + Send + 'static,
        >,
    > {
        Box::pin(async move { todo!() })
    }
}
