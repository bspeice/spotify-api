//! All kinds of page object
///Basic page
///ppaging abject(https://developer.spotify.com/web-api/object-model/#paging-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
}

#[cfg(feature = "api")]
mod pager {
    use crate::api::pager::Pager;
    use crate::api::SpotifyClient;
    use crate::model::page::Page;
    use crate::Result;
    use http_types::Url;

    impl<T: Unpin> Page<T> {
        pub fn into_stream<'a, C: SpotifyClient>(self, client: &'a C) -> Result<Pager<'a, C, T>> {
            let next = if let Some(next) = self.next {
                Some(Url::parse(&next)?)
            } else {
                None
            };
            Ok(Pager::with_items(client, self.items, next))
        }
    }
}

// TODO: IntoStream for `CursorBasedPage<T>`
// Will require some kind of trait to handle Page vs. CursorBasedPage vs. al the other CursorBased*
// objects used (see artist::CursorPageFullArtists)

/// cursor based page
///[cursor based paging object](https://developer.spotify.com/web-api/object-model/#cursor-based-paging-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CursorBasedPage<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Cursor,
    ///absent if it has read all data items. This field doesn't match what
    /// Spotify document says
    pub total: Option<u32>,
}

///Cursor object
///[cursor object](https://developer.spotify.com/web-api/object-model/#cursor-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cursor {
    pub after: Option<String>,
}
