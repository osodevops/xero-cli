use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::profiles::Profile;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub default: DefaultSection,

    #[serde(default)]
    pub auth: AuthSection,

    #[serde(default)]
    pub cache: CacheSection,

    #[serde(default)]
    pub rate_limit: RateLimitSection,

    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSection {
    pub active_profile: Option<String>,
    #[serde(default = "default_output_format")]
    pub output_format: String,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

impl Default for DefaultSection {
    fn default() -> Self {
        Self {
            active_profile: None,
            output_format: default_output_format(),
            page_size: default_page_size(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSection {
    pub client_id: Option<String>,
    #[serde(default = "default_callback_port")]
    pub callback_port: u16,
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
}

impl Default for AuthSection {
    fn default() -> Self {
        Self {
            client_id: None,
            callback_port: default_callback_port(),
            scopes: default_scopes(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSection {
    #[serde(default = "default_cache_enabled")]
    pub enabled: bool,
    #[serde(default = "default_cache_ttl_secs")]
    pub ttl_secs: u64,
    #[serde(default = "default_cache_list_ttl")]
    pub list_ttl_secs: u64,
    #[serde(default = "default_cache_get_ttl")]
    pub get_ttl_secs: u64,
    #[serde(default = "default_cache_max_size_mb")]
    pub max_size_mb: u64,
    pub directory: Option<String>,
}

impl Default for CacheSection {
    fn default() -> Self {
        Self {
            enabled: default_cache_enabled(),
            ttl_secs: default_cache_ttl_secs(),
            list_ttl_secs: default_cache_list_ttl(),
            get_ttl_secs: default_cache_get_ttl(),
            max_size_mb: default_cache_max_size_mb(),
            directory: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitSection {
    #[serde(default = "default_calls_per_minute")]
    pub calls_per_minute: u32,
    #[serde(default = "default_daily_limit")]
    pub daily_limit: u64,
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: u32,
}

impl Default for RateLimitSection {
    fn default() -> Self {
        Self {
            calls_per_minute: default_calls_per_minute(),
            daily_limit: default_daily_limit(),
            max_concurrent: default_max_concurrent(),
        }
    }
}

fn default_output_format() -> String {
    "table".to_string()
}
fn default_page_size() -> u32 {
    100
}
fn default_callback_port() -> u16 {
    8080
}
fn default_scopes() -> Vec<String> {
    vec![
        "openid".to_string(),
        "offline_access".to_string(),
        "accounting.invoices.read".to_string(),
        "accounting.payments.read".to_string(),
        "accounting.banktransactions.read".to_string(),
        "accounting.contacts.read".to_string(),
        "accounting.settings.read".to_string(),
        "accounting.reports.profitandloss.read".to_string(),
        "accounting.reports.balancesheet.read".to_string(),
    ]
}
fn default_cache_enabled() -> bool {
    true
}
fn default_cache_ttl_secs() -> u64 {
    300
}
fn default_cache_list_ttl() -> u64 {
    300
}
fn default_cache_get_ttl() -> u64 {
    900
}
fn default_cache_max_size_mb() -> u64 {
    50
}
fn default_calls_per_minute() -> u32 {
    60
}
fn default_daily_limit() -> u64 {
    5000
}
fn default_max_concurrent() -> u32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_toml() {
        let toml_str = r#"
[default]
output_format = "json"
"#;
        let config: ConfigFile = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default.output_format, "json");
        assert_eq!(config.default.page_size, 100);
    }

    #[test]
    fn parse_full_toml() {
        let toml_str = r#"
[default]
active_profile = "production"
output_format = "csv"
page_size = 50

[auth]
client_id = "abc123"
callback_port = 9090
scopes = ["openid", "offline_access"]

[cache]
enabled = false
ttl_secs = 600

[rate_limit]
calls_per_minute = 30
daily_limit = 2500
max_concurrent = 3

[profiles.production]
tenant_id = "tenant-123"
org_name = "My Company"
scopes = ["openid", "offline_access", "accounting.invoices"]
"#;
        let config: ConfigFile = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default.active_profile.as_deref(), Some("production"));
        assert_eq!(config.auth.callback_port, 9090);
        assert!(!config.cache.enabled);
        assert_eq!(config.rate_limit.calls_per_minute, 30);
        assert!(config.profiles.contains_key("production"));
    }

    #[test]
    fn parse_empty_toml() {
        let config: ConfigFile = toml::from_str("").unwrap();
        assert_eq!(config.default.output_format, "table");
        assert_eq!(config.default.page_size, 100);
        assert!(config.auth.client_id.is_none());
    }

    #[test]
    fn roundtrip_serialization() {
        let config = ConfigFile::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: ConfigFile = toml::from_str(&serialized).unwrap();
        assert_eq!(config.default.page_size, deserialized.default.page_size);
    }
}
