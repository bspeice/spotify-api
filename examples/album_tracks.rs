use futures::stream::StreamExt;
use http_client::h1::H1Client;
use spotify_api::api::albums::album_tracks;
use spotify_api::client::BasicSpotifyClient;
use spotify_api::model::track::SimplifiedTrack;
use spotify_api::oauth::FileCache;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "spotify-album-tracks",
    about = "Demonstration of retrieving tracks from a specific album."
)]
struct Opt {
    /// Path to the Spotify authorization token
    #[structopt(short, long, parse(from_os_str))]
    token_path: PathBuf,

    /// Album ID provided by Spotify
    #[structopt(short = "i", long)]
    album_id: String,
}

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let Opt {
        token_path,
        album_id,
    } = Opt::from_args();

    let token_cache = FileCache::new(token_path)?;
    let client = BasicSpotifyClient::new(H1Client::new(), token_cache);

    // While we _can_ retrieve just the first page, we can also get a streaming iterator
    // that resolves all tracks in the next pages.
    let page = album_tracks(&client, &album_id).await?;
    let mut stream = page.into_stream(&client)?;

    while let Some(track) = stream.next().await {
        let track: SimplifiedTrack = track?;
        let s = serde_json::to_string(&track)?;
        println!("{}", &s);
    }

    Ok(())
}
