use crate::api::{ClientExt, SpotifyClient};
use crate::model::album::SimplifiedAlbum;
use crate::model::artist::{FullArtist, FullArtists};
use crate::model::page::Page;
use crate::model::track::FullTracks;
use crate::Result;
use http_types::{Method, Request, Url};
use std::borrow::Borrow;

/// Get Spotify catalog information for a single artist identified by their unique Spotify ID.
///
/// https://developer.spotify.com/documentation/web-api/reference/artists/get-artist/
pub async fn artist<C: SpotifyClient + ?Sized>(client: &C, id: &str) -> Result<FullArtist> {
    let url = format!("https://api.spotify.com/v1/artists/{}", id);
    let url = Url::parse(&url)?;

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn artist_albums_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    include_groups: Option<&str>,
    country: Option<&str>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Page<SimplifiedAlbum>> {
    let url = format!("https://api.spotify.com/v1/artists/{}/albums", id);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, include_groups);
    set_query_param!(url, country);
    set_query_param!(url, limit);
    set_query_param!(url, offset);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn artist_albums<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
) -> Result<Page<SimplifiedAlbum>> {
    artist_albums_with_options(client, id, None, None, None, None).await
}

pub async fn top_tracks_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    country: Option<&str>,
) -> Result<FullTracks> {
    let url = format!("https://api.spotify.com/v1/artists/{}/top-tracks", id);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, country);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn top_tracks<C: SpotifyClient + ?Sized>(client: &C, id: &str) -> Result<FullTracks> {
    top_tracks_with_options(client, id, None).await
}

pub async fn related_artists<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
) -> Result<FullArtists> {
    let url = format!("https://api.spotify.com/v1/artists/{}/related-artists", id);
    let url = Url::parse(&url)?;

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn artists<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<FullArtists> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/artists").unwrap();
    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}
