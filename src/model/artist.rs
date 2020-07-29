//! All objects related to artist defined by Spotify API

use super::image::Image;
use super::page::CursorBasedPage;
use super::senum::Type;
use serde_json::Value;
use std::collections::HashMap;
///[artist object simplified](https://developer.spotify.com/web-api/object-model/#artist-object-simplified)
/// Simplified Artist Object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedArtist {
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub uri: Option<String>,
}

///[artist object full](https://developer.spotify.com/web-api/object-model/#artist-object-full)
/// Full Artist Object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullArtist {
    pub external_urls: HashMap<String, String>,
    pub followers: HashMap<String, Option<Value>>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    #[serde(rename = "type")]
    pub type_: Type,
    pub uri: String,
}

/// Full artist vector
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullArtists {
    // TODO: Think this should be Option<FullArtist>?
    // As the docs state - if an object isn't found, null values will be stored in the relevant
    // entry position.
    pub artists: Vec<FullArtist>,
}

/// Full Artists vector wrapped by cursor-based-page object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CursorPageFullArtists {
    pub artists: CursorBasedPage<FullArtist>,
}
