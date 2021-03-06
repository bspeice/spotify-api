//! All kinds of user object
use serde_json::Value;

use std::collections::HashMap;

use super::image::Image;
use super::senum::Type;
///[public user object](https://developer.spotify.com/web-api/object-model/#user-object-public)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    #[serde(rename = "type")]
    pub type_: Type,
    pub uri: String,
}

///[private user object](https://developer.spotify.com/web-api/object-model/#user-object-private)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateUser {
    pub country: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    #[serde(rename = "type")]
    pub type_: Type,
    pub uri: String,
}
