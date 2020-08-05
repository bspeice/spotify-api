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

#[cfg(feature = "api")]
mod pager {
    use crate::api::pager::{Pageable, Pager};
    use crate::api::SpotifyClient;
    use crate::model::page::{CursorBasedPage, Page};
    use crate::Result;
    use http_types::Url;
    use serde::de::DeserializeOwned;

    // TODO: Should Pageable also be implemented for parent structs?
    // Right now you have to do `NewReleases.albums.into_stream()`,
    // not sure if it should change to `NewReleases.into_stream()`.

    impl<T: DeserializeOwned> Pageable<T> for Page<T> {
        fn next_url(&self) -> Option<&str> {
            self.next.as_ref().map(|s| s.as_ref())
        }

        fn into_items(self) -> Vec<T> {
            self.items
        }
    }

    impl<T: Unpin + DeserializeOwned> Page<T> {
        pub fn into_stream<'a, C: SpotifyClient>(
            self,
            client: &'a C,
        ) -> Result<Pager<'a, C, T, Page<T>>> {
            let next = if let Some(next) = self.next {
                Some(Url::parse(&next)?)
            } else {
                None
            };
            Ok(Pager::with_items(client, self.items, next))
        }
    }

    impl<T: DeserializeOwned> Pageable<T> for CursorBasedPage<T> {
        fn next_url(&self) -> Option<&str> {
            self.next.as_ref().map(|s| s.as_ref())
        }

        fn into_items(self) -> Vec<T> {
            self.items
        }
    }

    impl<T: Unpin + DeserializeOwned> CursorBasedPage<T> {
        pub fn into_stream<'a, C: SpotifyClient>(
            self,
            client: &'a C,
        ) -> Result<Pager<'a, C, T, CursorBasedPage<T>>> {
            let next = if let Some(next) = self.next {
                Some(Url::parse(&next)?)
            } else {
                None
            };
            Ok(Pager::with_items(client, self.items, next))
        }
    }
}