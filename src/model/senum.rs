use super::show;
use super::track;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("unrecognized enum value {0}")]
pub struct Unrecognized(String);

// TODO: Instead of implementing an `as_str` method, would `AsRef<str>` make more sense?

/// Album type - ‘album’, ‘single’, ‘appears_on’, ‘compilation’
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}
impl FromStr for AlbumType {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(AlbumType::Album),
            "single" => Ok(AlbumType::Single),
            "appears_on" => Ok(AlbumType::AppearsOn),
            "compilation" => Ok(AlbumType::Compilation),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}
impl AlbumType {
    pub fn as_str(&self) -> &str {
        match *self {
            AlbumType::Album => "album",
            AlbumType::Single => "single",
            AlbumType::AppearsOn => "appears_on",
            AlbumType::Compilation => "compilation",
        }
    }
}

///  Type: ‘artist’, ‘album’,‘track’, ‘playlist’, 'show' or 'episode'
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Artist,
    Album,
    Track,
    Playlist,
    User,
    Show,
    Episode,
}
impl Type {
    pub fn as_str(&self) -> &str {
        match *self {
            Type::Album => "album",
            Type::Artist => "artist",
            Type::Track => "track",
            Type::Playlist => "playlist",
            Type::User => "user",
            Type::Show => "show",
            Type::Episode => "episode",
        }
    }
}
impl FromStr for Type {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(Type::Artist),
            "album" => Ok(Type::Album),
            "track" => Ok(Type::Track),
            "playlist" => Ok(Type::Playlist),
            "user" => Ok(Type::User),
            "show" => Ok(Type::Show),
            "episode" => Ok(Type::Episode),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}

/// additional_typs: track, episode
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalType {
    Track,
    Episode,
}
impl AdditionalType {
    pub fn as_str(&self) -> &str {
        match *self {
            AdditionalType::Track => "track",
            AdditionalType::Episode => "episode",
        }
    }
}
impl FromStr for AdditionalType {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(AdditionalType::Track),
            "episode" => Ok(AdditionalType::Episode),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}
/// currently_playing_type: track, episode, ad, unknown.
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyPlayingType {
    Track,
    Episode,
    Advertisement,
    Unknown,
}
impl CurrentlyPlayingType {
    pub fn as_str(&self) -> &str {
        match *self {
            CurrentlyPlayingType::Track => "track",
            CurrentlyPlayingType::Episode => "episode",
            CurrentlyPlayingType::Advertisement => "ad",
            CurrentlyPlayingType::Unknown => "unknown",
        }
    }
}
impl FromStr for CurrentlyPlayingType {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(CurrentlyPlayingType::Track),
            "episode" => Ok(CurrentlyPlayingType::Episode),
            "ad" => Ok(CurrentlyPlayingType::Advertisement),
            "unknown" => Ok(CurrentlyPlayingType::Unknown),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}

/// disallow: interrupting_playback, pausing, resuming, seeking, skipping_next, skipping_prev, toggling_repeat_context, toggling_shuffle, toggling_repeat_track, transferring_playback
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DisallowKey {
    InterruptingPlayback,
    Pausing,
    Resuming,
    Seeking,
    SkippingNext,
    SkippingPrev,
    TogglingRepeatContext,
    TogglingShuffle,
    TogglingRepeatTrack,
    TransferringPlayback,
}
impl DisallowKey {
    pub fn as_str(&self) -> &str {
        match *self {
            DisallowKey::InterruptingPlayback => "interrupting_playback",
            DisallowKey::Pausing => "pausing",
            DisallowKey::Resuming => "resuming",
            DisallowKey::Seeking => "seeking",
            DisallowKey::SkippingNext => "skipping_next",
            DisallowKey::SkippingPrev => "skipping_prev",
            DisallowKey::TogglingRepeatContext => "toggling_repeat_context",
            DisallowKey::TogglingShuffle => "toggling_shuffle",
            DisallowKey::TogglingRepeatTrack => "toggling_repeat_track",
            DisallowKey::TransferringPlayback => "transferring_playback",
        }
    }
}
impl FromStr for DisallowKey {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interrupting_playback" => Ok(DisallowKey::InterruptingPlayback),
            "pausing" => Ok(DisallowKey::Pausing),
            "resuming" => Ok(DisallowKey::Resuming),
            "seeking" => Ok(DisallowKey::Seeking),
            "skipping_next" => Ok(DisallowKey::SkippingNext),
            "skipping_prev" => Ok(DisallowKey::SkippingPrev),
            "toggling_repeat_context" => Ok(DisallowKey::TogglingRepeatContext),
            "toggling_shuffle" => Ok(DisallowKey::TogglingShuffle),
            "toggling_repeat_track" => Ok(DisallowKey::TogglingRepeatTrack),
            "transferring_playback" => Ok(DisallowKey::TransferringPlayback),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}

/// time range: long-term, medium-term, short-term
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

impl TimeRange {
    pub fn as_str(&self) -> &str {
        match *self {
            TimeRange::LongTerm => "long_term",
            TimeRange::MediumTerm => "medium_term",
            TimeRange::ShortTerm => "short_term",
        }
    }
}

impl FromStr for TimeRange {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "long_term" => Ok(TimeRange::LongTerm),
            "medium_term" => Ok(TimeRange::MediumTerm),
            "short_term" => Ok(TimeRange::ShortTerm),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}

///repeat state: track, context or off.
/// - track will repeat the current track.
/// - context will repeat the current context.
/// - off will turn repeat off.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}
impl RepeatState {
    pub fn as_str(&self) -> &str {
        match *self {
            RepeatState::Off => "off",
            RepeatState::Track => "track",
            RepeatState::Context => "context",
        }
    }
}
impl FromStr for RepeatState {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(RepeatState::Off),
            "track" => Ok(RepeatState::Track),
            "context" => Ok(RepeatState::Context),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}

/// Type for include_external: audio
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IncludeExternal {
    Audio,
}
impl IncludeExternal {
    pub fn as_str(&self) -> &str {
        match *self {
            IncludeExternal::Audio => "audio",
        }
    }
}
impl FromStr for IncludeExternal {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audio" => Ok(IncludeExternal::Audio),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}

/// Type for search: artist, album, track, playlist, show, episode
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Artist,
    Album,
    Track,
    Playlist,
    Show,
    Episode,
}

impl SearchType {
    pub fn as_str(&self) -> &str {
        match *self {
            SearchType::Album => "album",
            SearchType::Artist => "artist",
            SearchType::Track => "track",
            SearchType::Playlist => "playlist",
            SearchType::Show => "show",
            SearchType::Episode => "episode",
        }
    }
}
impl FromStr for SearchType {
    type Err = Unrecognized;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(SearchType::Artist),
            "album" => Ok(SearchType::Album),
            "track" => Ok(SearchType::Track),
            "playlist" => Ok(SearchType::Playlist),
            "show" => Ok(SearchType::Show),
            "episode" => Ok(SearchType::Episode),
            _ => Err(Unrecognized(s.to_owned())),
        }
    }
}

/// Device Type: computer, smartphone, speaker, TV, etc.
/// See the [Spotify developer
/// docs](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/#device-types)
/// for more information, or in case we are missing a device type here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Speaker,
    TV,
    AVR,
    STB,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlayingItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
}

/// https://developer.spotify.com/documentation/web-api/reference/browse/get-recommendations/#tuneable-track-attributes
#[derive(Clone, Debug)]
pub enum TrackAttribute {
    Acousticness(f32),
    Danceability(f32),
    DurationMs(usize),
    Energy(f32),
    Instrumentalness(f32),
    Key(u8),
    Liveness(f32),
    Loudness(f32),
    Mode(u8),
    Popularity(u8),
    Speechiness(f32),
    Tempo(f32),
    TimeSignature(u8),
    Valence(f32),
}

impl TrackAttribute {
    pub fn fmt_prefixed(&self, prefix: &str) -> String {
        let param = match self {
            TrackAttribute::Acousticness(v) => format!("acousticness={}", v),
            TrackAttribute::Danceability(v) => format!("danceability={}", v),
            TrackAttribute::DurationMs(v) => format!("duration_ms={}", v),
            TrackAttribute::Energy(v) => format!("energy={}", v),
            TrackAttribute::Instrumentalness(v) => format!("instrumentalness={}", v),
            TrackAttribute::Key(v) => format!("key={}", v),
            TrackAttribute::Liveness(v) => format!("liveness={}", v),
            TrackAttribute::Loudness(v) => format!("loudness={}", v),
            TrackAttribute::Mode(v) => format!("mode={}", v),
            TrackAttribute::Popularity(v) => format!("popularity={}", v),
            TrackAttribute::Speechiness(v) => format!("speechiness={}", v),
            TrackAttribute::Tempo(v) => format!("tempo={}", v),
            TrackAttribute::TimeSignature(v) => format!("time_signature={}", v),
            TrackAttribute::Valence(v) => format!("valence={}", v),
        };

        // TODO: Is there a more efficient way to handle this?
        let mut p = prefix.to_owned();
        p.push_str(param.as_str());
        p
    }
}
