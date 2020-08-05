use crate::api::client::{ClientExt, SpotifyClient};
use crate::model::show::{FullEpisode, FullEpisodes};
use crate::Result;
use http_types::{Method, Request, Url};
use std::borrow::Borrow;

pub async fn episode_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    market: Option<&str>,
) -> Result<FullEpisode> {
    let url = format!("https://api.spotify.com/v1/episodes/{}", id);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn episode<C: SpotifyClient + ?Sized>(client: &C, id: &str) -> Result<FullEpisode> {
    episode_with_options(client, id, None).await
}

pub async fn episodes_with_options<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
    market: Option<&str>,
) -> Result<FullEpisodes> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/episodes").unwrap();

    set_query_param_joined!(url, ids);
    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn episodes<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<FullEpisodes> {
    episodes_with_options(client, ids, None).await
}
