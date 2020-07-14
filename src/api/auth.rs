use super::Result;
use crate::clock::Clock;
use crate::oauth::{ClientCredentials, Token};
use base64::encode;
use http_client::HttpClient;
use http_types::headers::AUTHORIZATION;
use http_types::{Body, Method, Request, Url};

// The Refresh response looks very similar to a standard `Token`, but because
// Spotify may exclude the refresh token (indicating it doesn't need to change)
// it is handled slightly differently.
#[derive(Debug, Clone, Deserialize)]
struct RefreshResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u16,
    refresh_token: Option<String>,
}

impl RefreshResponse {
    fn into_token(self, clock: impl Clock, refresh_token: &str) -> Token {
        Token::new(
            clock,
            self.access_token,
            self.token_type,
            self.expires_in,
            self.refresh_token
                .unwrap_or_else(|| refresh_token.to_owned()),
            self.scope,
        )
    }
}

#[derive(Debug, Serialize)]
struct RefreshRequestBody<'a> {
    grant_body: &'a str,
    refresh_token: &'a str,
}

pub async fn refresh_token(
    client: impl HttpClient,
    clock: impl Clock,
    credentials: &ClientCredentials,
    token: &Token,
) -> Result<Token> {
    // UNWRAP: Statically-known URL
    let url: Url = Url::parse("https://accounts.spotify.com/api/token").unwrap();
    let mut req = Request::new(Method::Post, url);

    let b64_id = encode(&credentials.client_id);
    let b64_secret = encode(&credentials.client_secret);
    req.insert_header(
        AUTHORIZATION,
        format!("{} {}:{}", token.token_type, b64_id, b64_secret),
    );

    let req_body = RefreshRequestBody {
        grant_body: "refresh_token",
        refresh_token: &token.refresh_token,
    };
    // TODO: Review error policy. Because the crate `Result` type can convert into `http_client::Error`,
    // and `http_client::Error` can convert from any `std::error::Error`, we can use the try operator here
    // even though `crate::Error` doesn't explicitly handle these types.
    req.set_body(Body::from_form(&req_body)?);

    let mut resp: http_client::Response = client.send(req).await?;
    let resp = resp.take_body().into_form::<RefreshResponse>().await?;

    Ok(resp.into_token(clock, &token.refresh_token))
}
