use http_client::HttpClient;
use std::future::Future;
use std::pin::Pin;

use crate::oauth::Token;

/// HTTP client for interactions with Spotify. Handles authorization, API timeout, etc.
#[derive(Debug)]
pub struct SpotifyClient<C> {
    client: C,
    token: Token,
}

impl<C: HttpClient> SpotifyClient<C> {
    pub fn new(client: C, token: Token) -> Self {
        SpotifyClient { client, token }
    }
}

impl<C: HttpClient> HttpClient for SpotifyClient<C> {
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
