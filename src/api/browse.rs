use crate::api::client::{ClientExt, SpotifyClient};
use crate::model::album::NewReleases;
use crate::model::category::Category;
use crate::model::page::Page;
use crate::model::playlist::{FeaturedPlaylists, SimplifiedPlaylist};
use crate::model::recommend::Recommendations;
use crate::Result;
use http_types::{Method, Request, Url};
use std::{collections::HashMap, borrow::Borrow};

pub async fn category_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    category_id: &str,
    country: Option<&str>,
    locale: Option<&str>,
) -> Result<Category> {
    let url = format!(
        "https://api.spotify.com/v1/browse/categories/{}",
        category_id
    );
    let mut url = Url::parse(&url)?;

    set_query_param!(url, country);
    set_query_param!(url, locale);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn category<C: SpotifyClient + ?Sized>(
    client: &C,
    category_id: &str,
) -> Result<Category> {
    category_with_options(client, category_id, None, None).await
}

pub async fn category_playlists_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    category_id: &str,
    country: Option<&str>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Page<SimplifiedPlaylist>> {
    let url = format!(
        "https://api.spotify.com/v1/browse/categories/{}/playlists",
        category_id
    );
    let mut url = Url::parse(&url)?;

    set_query_param!(url, country);
    set_query_param!(url, limit);
    set_query_param!(url, offset);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn category_playlists<C: SpotifyClient + ?Sized>(
    client: &C,
    category_id: &str,
) -> Result<Page<SimplifiedPlaylist>> {
    category_playlists_with_options(client, category_id, None, None, None).await
}

pub async fn categories_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    country: Option<&str>,
    locale: Option<&str>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Page<Category>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/browse/categories").unwrap();

    set_query_param!(url, country);
    set_query_param!(url, locale);
    set_query_param!(url, limit);
    set_query_param!(url, offset);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn categories<C: SpotifyClient + ?Sized>(client: &C) -> Result<Page<Category>> {
    categories_with_options(client, None, None, None, None).await
}

// TODO: Use an actual timestamp object
pub async fn featured_playlists_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    locale: Option<&str>,
    country: Option<&str>,
    timestamp: Option<&str>,
    limit: Option<&str>,
    offset: Option<&str>,
) -> Result<FeaturedPlaylists> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/browse/featured-playlists").unwrap();

    set_query_param!(url, locale);
    set_query_param!(url, country);
    set_query_param!(url, timestamp);
    set_query_param!(url, limit);
    set_query_param!(url, offset);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn featured_playlists<C: SpotifyClient + ?Sized>(
    client: &C,
) -> Result<FeaturedPlaylists> {
    featured_playlists_with_options(client, None, None, None, None, None).await
}

pub async fn new_releases_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    country: Option<&str>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<NewReleases> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/browse/new-releases").unwrap();

    set_query_param!(url, country);
    set_query_param!(url, limit);
    set_query_param!(url, offset);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn new_releases<C: SpotifyClient + ?Sized>(client: &C) -> Result<NewReleases> {
    new_releases_with_options(client, None, None, None).await
}


// TODO: Recommendations call
// Want a "track attributes" enum that can handle both what attributes are available
// and whether they're int or float. Can then pass in attributes as a list of enum,
// rather than a HashMap.
