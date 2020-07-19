use crate::api::client::ClientExt;
use crate::api::SpotifyClient;
use crate::model::album::FullAlbum;
use crate::model::page::Page;
use crate::model::track::SimplifiedTrack;
use crate::Result;
use http_types::{Method, Request, Url};

pub async fn album_with_options(
    client: &impl SpotifyClient,
    id: &str,
    market: Option<&str>,
) -> Result<FullAlbum> {
    let url = format!("https://api.spotify.com/v1/albums/{}", id);
    let mut url = Url::parse(&url)?;
    if let Some(m) = market {
        url.set_query(Some(&format!("market={}", m)));
    }

    let req = Request::new(Method::Get, url);
    client
        .send_authorized(req)
        .deserialize_response::<FullAlbum>()
        .await
}

pub async fn album(client: &impl SpotifyClient, id: &str) -> Result<FullAlbum> {
    album_with_options(client, id, None).await
}

pub async fn album_tracks<C: SpotifyClient>(
    client: &impl SpotifyClient,
    id: &str,
) -> Result<Page<SimplifiedTrack>> {
    let url = Url::parse(&format!("https://api.spotify.com/v1/albums/{}/tracks", id))?;
    let req = Request::new(Method::Get, url);
    client
        .send_authorized(req)
        .deserialize_response::<Page<SimplifiedTrack>>()
        .await
}
