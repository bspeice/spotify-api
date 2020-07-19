use futures::future::BoxFuture;
use http_client::HttpClient;
use serde::de::DeserializeOwned;

use crate::oauth::TokenCache;
use crate::Error;

pub type HttpClientResult = Result<http_client::Response, http_client::Error>;
pub type HttpClientFuture = BoxFuture<'static, HttpClientResult>;

/// Marker trait to indicate operations that need a client capable of handling Spotify client
/// client session details. Specifically, this client is responsible for setting authorization
/// headers, retry after cooldown, etc.
pub trait SpotifyClient: HttpClient {
    fn send_authorized(&self, req: http_client::Request) -> HttpClientFuture;
}

pub trait ClientExt {
    fn deserialize_response<T: DeserializeOwned>(self) -> BoxFuture<'static, Result<T, Error>>;
}

impl ClientExt for BoxFuture<'static, HttpClientResult> {
    fn deserialize_response<T: DeserializeOwned>(self) -> BoxFuture<'static, Result<T, Error>> {
        // NOTE: While I'd normally rather implement this as a first-class Future
        // instead of allocating with `Box::pin()`, because `Request.body_bytes()`
        // is an opaque future, we'd have to allocate a `Box::pin()` anyway to get
        // access to the internal bytes.
        // Ultimately: this has exactly the same allocations as writing a manual
        // `impl Future`, so might as well use the easier syntax.
        Box::pin(async move {
            let mut resp: http_client::Response = self.await?;
            let body = resp.body_bytes().await?;
            serde_json::from_slice::<T>(&body).map_err(|e| e.into())
        })
    }
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
