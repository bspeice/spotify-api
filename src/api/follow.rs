use crate::api::client::{ClientExt, SpotifyClient};
use crate::model::artist::FollowedArtists;
use crate::Result;
use http_types::{Method, Request, Url};
use std::borrow::Borrow;

async fn user_follows<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    type_: &str,
    ids: &[B],
) -> Result<Vec<bool>> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/following/contains").unwrap();

    url.set_query(Some(type_));
    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn user_follows_artists<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<Vec<bool>> {
    user_follows(client, "type=artist", ids).await
}

pub async fn user_follows_users<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<Vec<bool>> {
    user_follows(client, "type=user", ids).await
}

pub async fn users_follow_playlist<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    playlist_id: &str,
    user_ids: &[B],
) -> Result<Vec<bool>> {
    let url = format!(
        "https://api.spotify.com/v1/playlists/{}/followers/contains",
        playlist_id
    );
    let mut url = Url::parse(&url)?;

    let ids = user_ids;
    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

async fn add_follows<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    type_: &str,
    ids: &[B],
) -> Result<()> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/following").unwrap();

    url.set_query(Some(type_));
    set_query_param_joined!(url, ids);

    let req = Request::new(Method::Put, url);
    client.send_authorized(req).await?;
    Ok(())
}

pub async fn follow_artists<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    add_follows(client, "type=artist", ids).await
}

pub async fn follow_users<C: SpotifyClient + ?Sized, B: Borrow<str>>(
    client: &C,
    ids: &[B],
) -> Result<()> {
    add_follows(client, "type=user", ids).await
}

pub async fn follow_playlist_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    playlist_id: &str,
    public: Option<bool>,
) -> Result<()> {
    let url = format!(
        "https://api.spotify.com/v1/playlists/{}/followers",
        playlist_id
    );
    let mut url = Url::parse(&url)?;

    set_query_param!(url, public);

    let req = Request::new(Method::Put, url);
    client.send_authorized(req).await?;
    Ok(())
}

pub async fn follow_playlist<C: SpotifyClient + ?Sized>(
    client: &C,
    playlist_id: &str,
) -> Result<()> {
    follow_playlist_with_options(client, playlist_id, None).await
}

pub async fn user_followed_artists_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    limit: Option<usize>,
    after: Option<&str>,
) -> Result<FollowedArtists> {
    // UNWRAP: Known-valid URL
    let mut url = Url::parse("https://api.spotify.com/v1/me/following?type=artist").unwrap();

    set_query_param!(url, limit);
    set_query_param!(url, after);

    let req = Request::new(Method::Get, url);
    client.send_authorized(req).deserialize_response().await
}

pub async fn user_followed_artists<C: SpotifyClient + ?Sized>(
    client: &C,
) -> Result<FollowedArtists> {
    user_followed_artists_with_options(client, None, None).await
}
