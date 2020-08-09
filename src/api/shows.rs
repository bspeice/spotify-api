use crate::api::{ClientExt, SpotifyClient};
use crate::model::page::Page;
use crate::model::show::{FullShow, SeversalSimplifiedShows, SimplifiedEpisode};
use crate::Result;

use http_types::{Method, Request, Url};
use std::borrow::Borrow;

pub async fn show_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    market: Option<&str>,
) -> Result<FullShow> {
    let url = format!("https://api.spotify.com/v1/shows/{}", id);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn show<C: SpotifyClient + ?Sized>(client: &C, id: &str) -> Result<FullShow> {
    show_with_options(client, id, None).await
}

pub async fn shows_with_options<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
    market: Option<&str>,
) -> Result<SeversalSimplifiedShows> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/shows").unwrap();

    set_query_param!(url, market);
    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn shows<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<SeversalSimplifiedShows> {
    shows_with_options(client, ids, None).await
}

pub async fn show_episodes_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    limit: Option<usize>,
    offset: Option<usize>,
    market: Option<&str>,
) -> Result<Page<SimplifiedEpisode>> {
    let url = format!("https://api.spotify.com/v1/shows/{}/episodes", id);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, limit);
    set_query_param!(url, offset);
    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn show_episodes<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
) -> Result<Page<SimplifiedEpisode>> {
    show_episodes_with_options(client, id, None, None, None).await
}
