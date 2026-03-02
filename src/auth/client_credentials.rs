use crate::auth::TokenSet;
use crate::error::{Result, XeroCliError};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenResponse, TokenUrl};

const TOKEN_URL: &str = "https://identity.xero.com/connect/token";
// Client credentials flow doesn't use an auth URL, but oauth2 crate requires one
const AUTH_URL: &str = "https://login.xero.com/identity/connect/authorize";

pub async fn authenticate(
    client_id: &str,
    client_secret: &str,
    scopes: &[String],
) -> Result<TokenSet> {
    let oauth_client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_uri(
            AuthUrl::new(AUTH_URL.to_string()).map_err(|e| XeroCliError::auth(e.to_string()))?,
        )
        .set_token_uri(
            TokenUrl::new(TOKEN_URL.to_string()).map_err(|e| XeroCliError::auth(e.to_string()))?,
        );

    let scope_list: Vec<oauth2::Scope> = scopes
        .iter()
        .map(|s| oauth2::Scope::new(s.clone()))
        .collect();

    let http_client = reqwest::Client::new();
    let token_response = oauth_client
        .exchange_client_credentials()
        .add_scopes(scope_list)
        .request_async(&http_client)
        .await
        .map_err(|e| XeroCliError::auth(format!("Client credentials auth failed: {e}")))?;

    let expires_at = chrono::Utc::now()
        + token_response
            .expires_in()
            .map(|d| chrono::Duration::seconds(d.as_secs() as i64))
            .unwrap_or(chrono::Duration::minutes(30));

    Ok(TokenSet {
        access_token: token_response.access_token().secret().to_string(),
        refresh_token: None, // M2M flow has no refresh tokens
        expires_at,
        token_type: "Bearer".to_string(),
        scopes: scopes.to_vec(),
        tenant_id: None,
    })
}
