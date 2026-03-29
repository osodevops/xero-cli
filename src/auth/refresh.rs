use crate::auth::TokenSet;
use crate::error::{Result, XeroCliError};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RefreshToken, TokenResponse, TokenUrl};

const TOKEN_URL: &str = "https://identity.xero.com/connect/token";
const AUTH_URL: &str = "https://login.xero.com/identity/connect/authorize";

pub async fn refresh_token(
    client_id: &str,
    client_secret: Option<&str>,
    refresh_token: &str,
) -> Result<TokenSet> {
    let mut oauth_client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_auth_uri(
            AuthUrl::new(AUTH_URL.to_string()).map_err(|e| XeroCliError::auth(e.to_string()))?,
        )
        .set_token_uri(
            TokenUrl::new(TOKEN_URL.to_string()).map_err(|e| XeroCliError::auth(e.to_string()))?,
        );

    if let Some(secret) = client_secret {
        oauth_client = oauth_client.set_client_secret(ClientSecret::new(secret.to_string()));
    }

    let http_client = reqwest::Client::new();
    let token_response = oauth_client
        .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
        .request_async(&http_client)
        .await
        .map_err(|e| XeroCliError::auth(format!("Token refresh failed: {e}")))?;

    let expires_at = chrono::Utc::now()
        + token_response
            .expires_in()
            .map(|d| chrono::Duration::seconds(d.as_secs() as i64))
            .unwrap_or(chrono::Duration::minutes(30));

    Ok(TokenSet {
        access_token: token_response.access_token().secret().to_string(),
        refresh_token: token_response
            .refresh_token()
            .map(|t| t.secret().to_string()),
        expires_at,
        token_type: "Bearer".to_string(),
        scopes: vec![],  // Scopes are preserved from original auth
        tenant_id: None, // Preserved from original auth
    })
}
