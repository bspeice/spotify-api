//! All enums for rspotify
use std::error;
use std::fmt;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}
/// The kind of an error that can occur.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// This error occurs when no proper enum was found.
    NoEnum(String),
}
impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::NoEnum(_) => "no proper enum was found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::NoEnum(ref s) => write!(f, "can't find proper enum of `{:?}`", s),
        }
    }
}
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(AlbumType::Album),
            "single" => Ok(AlbumType::Single),
            "appears_on" => Ok(AlbumType::AppearsOn),
            "compilation" => Ok(AlbumType::Compilation),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(Type::Artist),
            "album" => Ok(Type::Album),
            "track" => Ok(Type::Track),
            "playlist" => Ok(Type::Playlist),
            "user" => Ok(Type::User),
            "show" => Ok(Type::Show),
            "episode" => Ok(Type::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(AdditionalType::Track),
            "episode" => Ok(AdditionalType::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(CurrentlyPlayingType::Track),
            "episode" => Ok(CurrentlyPlayingType::Episode),
            "ad" => Ok(CurrentlyPlayingType::Advertisement),
            "unknown" => Ok(CurrentlyPlayingType::Unknown),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
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
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "long_term" => Ok(TimeRange::LongTerm),
            "medium_term" => Ok(TimeRange::MediumTerm),
            "short_term" => Ok(TimeRange::ShortTerm),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(RepeatState::Off),
            "track" => Ok(RepeatState::Track),
            "context" => Ok(RepeatState::Context),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audio" => Ok(IncludeExternal::Audio),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(SearchType::Artist),
            "album" => Ok(SearchType::Album),
            "track" => Ok(SearchType::Track),
            "playlist" => Ok(SearchType::Playlist),
            "show" => Ok(SearchType::Show),
            "episode" => Ok(SearchType::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
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
