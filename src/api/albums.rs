use crate::api::client::ClientExt;
use crate::api::SpotifyClient;
use crate::model::album::{FullAlbum, FullAlbums};
use crate::model::page::Page;
use crate::model::track::SimplifiedTrack;
use crate::Result;
use http_types::{Method, Request, Url};
use std::borrow::Borrow;

pub async fn album_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    market: Option<&str>,
) -> Result<FullAlbum> {
    let url = format!("https://api.spotify.com/v1/albums/{}", id);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn album<C: SpotifyClient + ?Sized>(client: &C, id: &str) -> Result<FullAlbum> {
    album_with_options(client, id, None).await
}

pub async fn album_tracks_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    limit: Option<usize>,
    offset: Option<usize>,
    market: Option<&str>,
) -> Result<Page<SimplifiedTrack>> {
    let mut url = Url::parse(&format!("https://api.spotify.com/v1/albums/{}/tracks", id))?;

    set_query_param!(url, limit);
    set_query_param!(url, offset);
    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn album_tracks<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
) -> Result<Page<SimplifiedTrack>> {
    album_tracks_with_options(client, id, None, None, None).await
}

pub async fn albums_with_options<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
    market: Option<&str>,
) -> Result<FullAlbums> {
    // TODO: reject if more than 20 IDs? Or let the users handle that?
    let mut url = Url::parse(&format!("https://api.spotify.com/v1/albums"))?;
    set_query_param!(url, market);
    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn albums<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<FullAlbums> {
    albums_with_options(client, ids, None).await
}
