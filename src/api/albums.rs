use crate::api::client::ClientExt;
use crate::api::SpotifyClient;
use crate::model::album::FullAlbum;
use crate::model::page::Page;
use crate::model::track::SimplifiedTrack;
use crate::Result;
use http_types::{Method, Request, Url};

pub async fn album_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
    market: Option<&str>,
) -> Result<FullAlbum> {
    let url = format!("https://api.spotify.com/v1/albums/{}", id);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, market);

    let req = Request::new(Method::Get, url);
    client
        .send_authorized(req)
        .deserialize_response::<FullAlbum>()
        .await
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
    client
        .send_authorized(req)
        .deserialize_response::<Page<SimplifiedTrack>>()
        .await
}

pub async fn album_tracks<C: SpotifyClient + ?Sized>(
    client: &C,
    id: &str,
) -> Result<Page<SimplifiedTrack>> {
    album_tracks_with_options(client, id, None, None, None).await
}
