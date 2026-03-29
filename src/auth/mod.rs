pub mod client_credentials;
pub mod pkce;
pub mod refresh;
pub mod token_store;

use crate::error::{Result, XeroCliError};
use serde::{Deserialize, Serialize};
use token_store::TokenStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub token_type: String,
    pub scopes: Vec<String>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "tenantName")]
    pub tenant_name: Option<String>,
    #[serde(rename = "tenantType")]
    pub tenant_type: String,
}

impl TokenSet {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }

    pub fn should_refresh(&self) -> bool {
        let refresh_at = self.expires_at - chrono::Duration::minutes(5);
        chrono::Utc::now() >= refresh_at
    }

    pub fn time_remaining(&self) -> chrono::Duration {
        self.expires_at - chrono::Utc::now()
    }
}

pub async fn ensure_authenticated(
    store: &TokenStore,
    client_id: &str,
    client_secret: Option<&str>,
) -> Result<TokenSet> {
    let tokens = store
        .load()
        .map_err(|_| XeroCliError::auth("No stored credentials found"))?;

    if tokens.is_expired() {
        if let Some(ref refresh_token) = tokens.refresh_token {
            tracing::info!("Access token expired, refreshing...");
            let mut new_tokens =
                refresh::refresh_token(client_id, client_secret, refresh_token).await?;
            // Preserve tenant_id and scopes from original auth
            if new_tokens.tenant_id.is_none() {
                new_tokens.tenant_id = tokens.tenant_id.clone();
            }
            if new_tokens.scopes.is_empty() {
                new_tokens.scopes = tokens.scopes.clone();
            }
            store.save(&new_tokens)?;
            return Ok(new_tokens);
        }
        return Err(XeroCliError::auth(
            "Access token expired and no refresh token available. Run `xero auth login`.",
        ));
    }

    if tokens.should_refresh() {
        if let Some(ref refresh_token) = tokens.refresh_token {
            tracing::debug!(
                "Pre-emptively refreshing token (expires in {}s)",
                tokens.time_remaining().num_seconds()
            );
            match refresh::refresh_token(client_id, client_secret, refresh_token).await {
                Ok(mut new_tokens) => {
                    if new_tokens.tenant_id.is_none() {
                        new_tokens.tenant_id = tokens.tenant_id.clone();
                    }
                    if new_tokens.scopes.is_empty() {
                        new_tokens.scopes = tokens.scopes.clone();
                    }
                    store.save(&new_tokens)?;
                    return Ok(new_tokens);
                }
                Err(e) => {
                    tracing::warn!("Pre-emptive refresh failed, using existing token: {e}");
                }
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(minutes_until_expiry: i64) -> TokenSet {
        TokenSet {
            access_token: "test_access".to_string(),
            refresh_token: Some("test_refresh".to_string()),
            expires_at: chrono::Utc::now() + chrono::Duration::minutes(minutes_until_expiry),
            token_type: "Bearer".to_string(),
            scopes: vec!["openid".to_string()],
            tenant_id: None,
        }
    }

    #[test]
    fn token_not_expired() {
        let token = make_token(30);
        assert!(!token.is_expired());
    }

    #[test]
    fn token_is_expired() {
        let token = make_token(-1);
        assert!(token.is_expired());
    }

    #[test]
    fn token_should_refresh_within_5_minutes() {
        let token = make_token(4);
        assert!(token.should_refresh());
    }

    #[test]
    fn token_should_not_refresh_when_fresh() {
        let token = make_token(25);
        assert!(!token.should_refresh());
    }
}
