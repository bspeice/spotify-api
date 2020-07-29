use crate::api::client::{ClientExt, SpotifyClient};
use crate::model::album::NewReleases;
use crate::model::category::Category;
use crate::model::page::Page;
use crate::model::playlist::{FeaturedPlaylists, SimplifiedPlaylist};
use crate::model::recommend::Recommendations;
use crate::model::senum::TrackAttribute;
use crate::Result;
use http_types::{Method, Request, Url};
use std::borrow::Borrow;

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

pub async fn recommendations_with_options<
    C: SpotifyClient + ?Sized,
    B1: Borrow<str>,
    B2: Borrow<str>,
    B3: Borrow<str>,
>(
    client: &C,
    limit: Option<usize>,
    market: Option<&str>,
    max_attributes: &[TrackAttribute],
    min_attributes: &[TrackAttribute],
    target_attributes: &[TrackAttribute],
    seed_artists: Option<&[B1]>,
    seed_genres: Option<&[B2]>,
    seed_tracks: Option<&[B3]>,
) -> Result<Recommendations> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/recommendations").unwrap();

    set_query_param!(url, limit);
    set_query_param!(url, market);

    for a in max_attributes {
        let param = a.fmt_prefixed("max_");
        url.set_query(Some(param.as_str()));
    }

    for a in min_attributes {
        let param = a.fmt_prefixed("min_");
        url.set_query(Some(param.as_str()));
    }

    for a in target_attributes {
        let param = a.fmt_prefixed("target_");
        url.set_query(Some(param.as_str()));
    }

    if let Some(seed_artists) = seed_artists {
        set_query_param_joined!(url, seed_artists);
    }

    if let Some(seed_genres) = seed_genres {
        set_query_param_joined!(url, seed_genres);
    }

    if let Some(seed_tracks) = seed_tracks {
        set_query_param_joined!(url, seed_tracks);
    }

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}
