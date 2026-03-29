use crate::api::XeroClient;
use crate::auth::token_store::TokenStore;
use crate::auth::TokenSet;
use crate::cache::{CacheStore, CachedClient};
use crate::cli::GlobalArgs;
use crate::config::AppConfig;
use crate::error::{Result, XeroCliError};
use crate::rate_limit::budget::DailyBudget;
use crate::rate_limit::RateLimiter;
use std::sync::Arc;

pub async fn build_client(global: &GlobalArgs) -> Result<CachedClient> {
    let config = AppConfig::load(global.config_path.as_deref())?;
    let config_dir = crate::config::default_config_dir()?;

    let store = TokenStore::new(config_dir);

    let env_client_id = std::env::var("XERO_CLIENT_ID").ok();
    let client_id = config
        .config_file
        .auth
        .client_id
        .as_deref()
        .or(env_client_id.as_deref())
        .unwrap_or_default()
        .to_string();

    let client_secret = std::env::var("XERO_CLIENT_SECRET").ok();

    // Check for direct access token override (for testing/CI)
    let tokens = if let Ok(token) = std::env::var("XERO_ACCESS_TOKEN") {
        TokenSet {
            access_token: token,
            refresh_token: None,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            token_type: "Bearer".to_string(),
            scopes: vec![],
            tenant_id: std::env::var("XERO_TENANT_ID").ok(),
        }
    } else {
        crate::auth::ensure_authenticated(&store, &client_id, client_secret.as_deref()).await?
    };

    let tenant_id = global
        .profile
        .as_deref()
        .and_then(|p| config.active_profile(Some(p)))
        .map(|p| p.tenant_id.clone())
        .or(tokens.tenant_id.clone())
        .or_else(|| std::env::var("XERO_TENANT_ID").ok())
        .ok_or_else(|| {
            XeroCliError::config("No tenant ID found. Run `xero auth login` or set XERO_TENANT_ID")
        })?;

    let rate_limiter = Arc::new(RateLimiter::new(
        config.config_file.rate_limit.calls_per_minute,
        config.config_file.rate_limit.max_concurrent,
    ));
    let daily_budget = Arc::new(DailyBudget::new(config.config_file.rate_limit.daily_limit));

    let xero_client = XeroClient::new(tokens.access_token, tenant_id, rate_limiter, daily_budget);

    // Set up cache if enabled
    let cache = if !global.no_cache && config.config_file.cache.enabled {
        let cache_dir = if let Some(ref dir) = config.config_file.cache.directory {
            std::path::PathBuf::from(dir)
        } else {
            crate::config::default_cache_dir()?
        };
        CacheStore::new(&cache_dir).ok().map(Arc::new)
    } else {
        None
    };

    Ok(CachedClient::new(
        xero_client,
        cache,
        config.config_file.cache.list_ttl_secs,
        config.config_file.cache.get_ttl_secs,
    ))
}
