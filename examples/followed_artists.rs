use futures::stream::StreamExt;
use http_client::h1::H1Client;
use spotify_api::api::follow::user_followed_artists;
use spotify_api::client::BasicSpotifyClient;
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
}

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let Opt {
        token_path,
    } = Opt::from_args();

    let token_cache = FileCache::new(token_path)?;
    let client = BasicSpotifyClient::new(H1Client::new(), token_cache);

    // While we _can_ retrieve just the first page, we can also get a streaming iterator
    // that resolves all tracks in the next pages.
    let followed = user_followed_artists(&client).await?;
    let mut stream = followed.artists.into_stream(&client)?;

    while let Some(artist) = stream.next().await {
        let artist = artist?;
        let s = serde_json::to_string(&artist)?;
        println!("{}", &s);
    }

    Ok(())
}