use super::Result;
use crate::api::client::ClientExt;
use crate::clock::Clock;
use crate::oauth::{ClientCredentials, Token};
use http_client::HttpClient;
use http_types::headers::AUTHORIZATION;
use http_types::{Body, Method, Request, StatusCode, Url};

#[derive(Debug, Clone, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u16,
    // Optional because refresh responses may not include a new refresh token
    refresh_token: Option<String>,
}

impl TokenResponse {
    fn try_into_token<C: Clock + ?Sized>(self, clock: &C) -> Result<Token> {
        let refresh_token = self.refresh_token.ok_or_else(|| {
            http_types::Error::from_str(
                StatusCode::InternalServerError,
                "no refresh token was provided",
            )
        })?;
        Ok(Token::new(
            clock,
            self.access_token,
            self.token_type,
            self.expires_in,
            refresh_token,
            self.scope,
        ))
    }

    fn into_token<C: Clock + ?Sized>(self, clock: &C, refresh_token: &str) -> Token {
        let refresh_token = self
            .refresh_token
            .unwrap_or_else(|| refresh_token.to_owned());
        Token::new(
            clock,
            self.access_token,
            self.token_type,
            self.expires_in,
            refresh_token,
            self.scope,
        )
    }
}

/// Build the authorization URL to retrieve a Spotify access token.
///
/// Users visiting this URL will choose whether to grant your application access. If that access
/// is granted, users will be redirected according to the `redirect_uri`, and the new URI will
/// contain a `code` query parameter that can be exchanged for an access token.
pub fn authorize_url(
    credentials: &ClientCredentials,
    state: Option<&str>,
    scope: Option<&str>,
    show_dialog: Option<bool>,
) -> Result<http_types::Url> {
    // TODO: This could probably be done without so many intermediate allocations
    let client_id = format!("&client_id={}", &credentials.client_id);
    let redirect_uri = format!("&redirect_uri={}", &credentials.redirect_uri);
    let state = state.map(|s| format!("&state={}", s)).unwrap_or_default();
    let scope = scope.map(|s| format!("&scope={}", s)).unwrap_or_default();
    let show_dialog = show_dialog
        .map(|s| format!("&show_dialog={}", s))
        .unwrap_or_default();

    let mut url = "https://accounts.spotify.com/authorize?response_type=code".to_owned();
    url.push_str(&client_id);
    url.push_str(&redirect_uri);
    url.push_str(&state);
    url.push_str(&scope);
    url.push_str(&show_dialog);

    http_types::Url::parse(&url).map_err(|e| e.into())
}

#[derive(Debug, Serialize)]
struct AuthorizeRequestBody<'a> {
    grant_type: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
}

pub async fn authorize(
    client: &impl HttpClient,
    clock: &impl Clock,
    credentials: &ClientCredentials,
    code: &str,
) -> Result<Token> {
    // UNWRAP: Statically-known URL
    let url = Url::parse("https://accounts.spotify.com/api/token").unwrap();

    let req_body = AuthorizeRequestBody {
        grant_type: "authorization_code",
        code,
        redirect_uri: &credentials.redirect_uri,
    };
    let body = http_types::Body::from_form(&req_body)?;

    let mut request = http_types::Request::new(Method::Post, url);
    request.insert_header(AUTHORIZATION, credentials.authorization_header());
    request.set_body(body);

    client
        .send(request)
        .deserialize_response::<TokenResponse>()
        .await?
        .try_into_token(clock)
}

#[derive(Debug, Serialize)]
struct RefreshRequestBody<'a> {
    grant_type: &'a str,
    refresh_token: &'a str,
}

pub async fn refresh(
    client: &impl HttpClient,
    clock: &impl Clock,
    credentials: &ClientCredentials,
    token: &Token,
) -> Result<Token> {
    // UNWRAP: Statically-known URL
    let url = Url::parse("https://accounts.spotify.com/api/token").unwrap();
    let mut req = Request::new(Method::Post, url);
    req.insert_header(AUTHORIZATION, credentials.authorization_header());

    let req_body = RefreshRequestBody {
        grant_type: "refresh_token",
        refresh_token: &token.refresh_token,
    };
    // UNWRAP: Encoding a form with two string fields guaranteed to succeed
    // (Specifically, URL-encoding is guaranteed to succeed)
    req.set_body(Body::from_form(&req_body).unwrap());

    client
        .send(req)
        .deserialize_response::<TokenResponse>()
        .await
        .map(|t| t.into_token(clock, &token.refresh_token))
}
