use async_std::task;
use http_client::h1::H1Client;
use http_types::Url;
use spotify_api::api::auth::{authorize, authorize_url, refresh};
use spotify_api::clock::SystemClock;
use spotify_api::oauth::{ClientCredentials, FileCache, TokenCache};
use std::borrow::Cow;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "spotify-authorize",
    about = "Demonstration of Spotify basic Authorization workflow."
)]
struct Opt {
    /// Path to store the authorization ticket for future use
    #[structopt(short, long, parse(from_os_str))]
    ticket_path: PathBuf,

    /// Client ID provided by Spotify
    #[structopt(short = "i", long)]
    client_id: String,

    /// Client secret provided by Spotify
    #[structopt(short = "s", long)]
    client_secret: String,

    /// Redirect URI provided to Spotify
    #[structopt(short = "r", long, parse(try_from_str))]
    redirect_uri: Url,
}

#[derive(Debug, Error)]
enum Error {
    #[error("redirect URL missing auth code")]
    MissingAuthCode,
}

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let Opt {
        ticket_path,
        client_id,
        client_secret,
        redirect_uri,
    } = Opt::from_args();

    let credentials = ClientCredentials::new(client_id, client_secret, redirect_uri.into_string());
    let mut token_cache = FileCache::new(ticket_path)?;

    // Force showing a dialog each time because this is a demonstration
    let auth_url = authorize_url(&credentials, None, None, Some(true))?;
    let auth_url = auth_url.into_string();

    println!("Opening browser to url={}\nAfter approving access, please paste the URL you were redirected to.", &auth_url);
    webbrowser::open(&auth_url)?;

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;

    let redirect = Url::parse(&buffer)?;

    let mut auth_code: Option<Cow<str>> = None;
    for (name, value) in redirect.query_pairs() {
        if name == "code" && auth_code.is_none() {
            auth_code.replace(value.clone());
        }
    }

    let auth_code = auth_code.ok_or(Error::MissingAuthCode)?;

    let client = H1Client::new();
    let clock = SystemClock::new();

    let token = authorize(&client, &clock, &credentials, &auth_code).await?;

    println!("Saving token");
    token_cache.update(token)?;

    println!("Waiting 2 seconds before refreshing token");
    task::sleep(Duration::from_secs(2)).await;

    println!("Refreshing auth token");
    // UNWRAP: We _just_ (successfully) saved a token
    let token = token_cache.current().unwrap();
    let token = refresh(&client, &clock, &credentials, token).await?;

    token_cache.update(token).map_err(|e| e.into())
}
