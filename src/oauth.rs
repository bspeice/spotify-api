use crate::clock::Clock;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl ClientCredentials {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        ClientCredentials {
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    #[cfg(feature = "api")]
    pub fn authorization_header(&self) -> String {
        // TODO: Is there a material benefit to computing string length ahead of time?
        // There's one unnecessary string allocation below, but I'm not sure what the
        // benefit would be given how weird the code would end up looking.
        let s = format!("{}:{}", self.client_id, self.client_secret);
        let s = base64::encode(&s);
        format!("Basic {}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub expires_at: u64,
    pub refresh_token: String,
    // TODO: Should there be a distinction between missing and empty scope?
    pub scope: String,
}

impl Token {
    pub fn new(
        clock: &impl Clock,
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

pub trait TokenCache: Debug {
    type Error;

    fn current(&self) -> Option<&Token>;
    fn update(&mut self, token: Token) -> Result<(), Self::Error>;
}

#[derive(Debug, Error)]
pub enum FileCacheError {
    #[error("")]
    Io(#[from] io::Error),

    #[error("")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct FileCache {
    path: PathBuf,
    token: Option<Token>,
}

impl FileCache {
    pub fn new(path: PathBuf) -> Result<Self, FileCacheError> {
        // Note: to avoid expressing a preference for a specific runtime, this implementation uses
        // synchronous I/O.

        // If the file doesn't exist yet, allow creation
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let mut token_bytes = Vec::new();
        file.read_to_end(&mut token_bytes)?;

        // If the file is empty or invalid, don't set the initial ticket
        let token = serde_json::from_slice(&token_bytes).unwrap_or_default();

        Ok(FileCache { path, token })
    }
}

impl TokenCache for FileCache {
    type Error = FileCacheError;

    fn current(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn update(&mut self, token: Token) -> Result<(), FileCacheError> {
        let mut file = OpenOptions::new().write(true).open(&self.path)?;
        let token_bytes = serde_json::to_vec(&token)?;

        file.write_all(&token_bytes)?;
        self.token.replace(token);

        Ok(())
    }
}
