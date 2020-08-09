use crate::api::client::{ClientExt, SpotifyClient};
use crate::model::album::FullAlbum;
use crate::model::page::Page;
use crate::model::senum::TimeRange;
use crate::model::track::FullTrack;
use crate::Result;

use http_types::{Method, Request, Url};
use serde::de::DeserializeOwned;

async fn top_recommendations<C: SpotifyClient + ?Sized, T: DeserializeOwned>(
    client: &C,
    type_: &str,
    limit: Option<usize>,
    offset: Option<usize>,
    time_range: Option<TimeRange>,
) -> Result<T> {
    let url = format!("https://api.spotify.com/v1/me/top/{}", type_);
    let mut url = Url::parse(&url)?;

    set_query_param!(url, limit);
    set_query_param!(url, offset);

    let time_range = time_range.as_ref().map(|t| t.as_str());
    set_query_param!(url, time_range);

    let req = Request::new(Method::Get, url);
    client
        .send_authorized(req)
        .deserialize_response::<T>()
        .await
}

pub async fn top_tracks_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    limit: Option<usize>,
    offset: Option<usize>,
    time_range: Option<TimeRange>,
) -> Result<Page<FullTrack>> {
    // It's stunning to me that type inference works here
    top_recommendations(client, "tracks", limit, offset, time_range).await
}

pub async fn top_tracks<C: SpotifyClient + ?Sized>(
    client: &C,
    time_range: Option<TimeRange>,
) -> Result<Page<FullTrack>> {
    top_tracks_with_options(client, None, None, time_range).await
}

pub async fn top_albums_with_options<C: SpotifyClient + ?Sized>(
    client: &C,
    limit: Option<usize>,
    offset: Option<usize>,
    time_range: Option<TimeRange>,
) -> Result<Page<FullAlbum>> {
    // It's stunning to me that type inference works here
    top_recommendations(client, "albums", limit, offset, time_range).await
}

pub async fn top_albums<C: SpotifyClient + ?Sized>(
    client: &C,
    time_range: Option<TimeRange>,
) -> Result<Page<FullAlbum>> {
    top_albums_with_options(client, None, None, time_range).await
}
