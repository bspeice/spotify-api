use crate::client::{ClientExt, SpotifyClient};
use crate::model::album::SavedAlbum;
use crate::model::page::Page;
use crate::model::show::Show;
use crate::model::track::SavedTrack;
use crate::Result;

use http_types::{Method, Request, Url};
use std::borrow::Borrow;

pub async fn library_contains_albums<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<Vec<bool>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/albums/contains").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn library_contains_shows<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<Vec<bool>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/shows/contains").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn library_contains_tracks<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<Vec<bool>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/tracks/contains").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn library_get_albums_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    limit: Option<usize>,
    offset: Option<usize>,
    market: Option<&str>,
) -> Result<Page<SavedAlbum>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/albums").unwrap();

    set_query_param!(url, limit);
    set_query_param!(url, offset);
    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

// Methods like this are why I appreciate overloading in C++
pub async fn library_get_albums<C: SpotifyClient + ?Sized>(client: &C) -> Result<Page<SavedAlbum>> {
    library_get_albums_with_options(client, None, None, None).await
}

pub async fn library_get_shows_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Page<Show>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/shows").unwrap();

    set_query_param!(url, limit);
    set_query_param!(url, offset);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn library_get_shows<C: SpotifyClient + ?Sized>(client: &C) -> Result<Page<Show>> {
    library_get_shows_with_options(client, None, None).await
}

pub async fn library_get_tracks_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    limit: Option<usize>,
    offset: Option<usize>,
    market: Option<&str>,
) -> Result<Page<SavedTrack>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/tracks").unwrap();

    set_query_param!(url, limit);
    set_query_param!(url, offset);
    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn library_get_tracks<C: SpotifyClient + ?Sized>(client: &C) -> Result<Page<SavedTrack>> {
    library_get_tracks_with_options(client, None, None, None).await
}

pub async fn library_remove_albums<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/albums").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Delete, url);
    client.send_authorized(req).await?;
    Ok(())
}

pub async fn library_remove_shows<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/shows").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Delete, url);
    client.send_authorized(req).await?;
    Ok(())
}

pub async fn library_remove_tracks<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/tracks").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Delete, url);
    client.send_authorized(req).await?;
    Ok(())
}

pub async fn library_save_albums<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/albums").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Put, url);
    client.send_authorized(req).await?;
    Ok(())
}

pub async fn library_save_shows<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/shows").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Put, url);
    client.send_authorized(req).await?;
    Ok(())
}

pub async fn library_save_tracks<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/tracks").unwrap();

    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Put, url);
    client.send_authorized(req).await?;
    Ok(())
}
