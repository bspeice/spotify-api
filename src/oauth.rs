use crate::clock::Clock;
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub expires_at: u64,
    pub refresh_token: String,
    pub scope: String,
}

impl Token {
    pub fn new(
        clock: impl Clock,
        access_token: String,
        token_type: String,
        expires_in: u16,
        refresh_token: String,
        scope: String,
    ) -> Self {
        // UNWRAP: Can fail only if `now - expires_in` is prior to Unix epoch
        let expires_at = (clock.now() + Duration::from_secs(expires_in as u64))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Token {
            access_token,
            token_type,
            expires_in,
            expires_at,
            refresh_token,
            scope,
        }
    }
}

pub enum Scope {
    AppRemoteControl,
    PlaylistModifyPrivate,
    PlaylistModifyPublic,
    PlaylistReadCollaborative,
    PlaylistReadPrivate,
    Streaming,
    UgcImageUpload,
    UserFollowRead,
    UserLibraryModify,
    UserLibraryRead,
    UserFollowModify,
    UserModifyPlaybackState,
    UserReadCurrentlyPlaying,
    UserReadEmail,
    UserReadPlaybackState,
    UserReadPlaybackPosition,
    UserReadPrivate,
    UserReadRecentlyPlayed,
    UserTopRead,
}

impl Scope {
    fn as_str(&self) -> &'static str {
        match self {
            Scope::AppRemoteControl => "app-remote-control",
            Scope::PlaylistModifyPrivate => "playlist-modify-private",
            Scope::PlaylistModifyPublic => "playlist-modify-public",
            Scope::PlaylistReadCollaborative => "playlist-read-collaborative",
            Scope::PlaylistReadPrivate => "playlist-read-private",
            Scope::Streaming => "streaming",
            Scope::UgcImageUpload => "ugc-image-upload",
            Scope::UserFollowRead => "user-follow-read",
            Scope::UserLibraryModify => "user-library-modify",
            Scope::UserLibraryRead => "user-library-read",
            Scope::UserFollowModify => "user-follow-modify",
            Scope::UserModifyPlaybackState => "user-modify-playback-state",
            Scope::UserReadCurrentlyPlaying => "user-read-currently-playing",
            Scope::UserReadEmail => "user-read-email",
            Scope::UserReadPlaybackState => "user-read-playback-state",
            Scope::UserReadPlaybackPosition => "user-read-playback-position",
            Scope::UserReadPrivate => "user-read-private",
            Scope::UserReadRecentlyPlayed => "user-read-recently-played",
            Scope::UserTopRead => "user-top-read",
        }
    }
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &'static str {
        self.as_str()
    }
}
