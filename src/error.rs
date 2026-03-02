use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum XeroCliError {
    #[error("Authentication failed: {message}")]
    #[diagnostic(code(xero::auth), help("Run `xero auth login` to authenticate"))]
    Auth { message: String },

    #[error("API error ({status}): {message}")]
    #[diagnostic(code(xero::api))]
    Api { status: u16, message: String },

    #[error("Rate limited — retry after {retry_after_secs}s")]
    #[diagnostic(
        code(xero::rate_limited),
        help("The Xero API rate limit (60 calls/min) has been reached. The CLI will automatically retry.")
    )]
    RateLimited { retry_after_secs: u64 },

    #[error("Configuration error: {message}")]
    #[diagnostic(
        code(xero::config),
        help("Check your config at ~/.config/xero-cli/config.toml or run `xero config init`")
    )]
    Config { message: String },

    #[error("Validation error: {message}")]
    #[diagnostic(code(xero::validation))]
    Validation { message: String },

    #[error("Missing required scope: {scope}")]
    #[diagnostic(
        code(xero::missing_scope),
        help("Run `xero auth scopes add {scope}` to add the required scope, then re-authenticate")
    )]
    MissingScope { scope: String },

    #[error("HTTP error: {0}")]
    #[diagnostic(code(xero::http))]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    #[diagnostic(code(xero::io))]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    #[diagnostic(code(xero::json))]
    Json(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    #[diagnostic(code(xero::toml))]
    Toml(#[from] toml::de::Error),

    #[error("Daily API budget exhausted ({used}/{limit} calls used)")]
    #[diagnostic(
        code(xero::budget_exhausted),
        help(
            "The daily limit of {limit} API calls has been reached. Budget resets at midnight UTC."
        )
    )]
    BudgetExhausted { used: u64, limit: u64 },
}

pub type Result<T> = std::result::Result<T, XeroCliError>;

impl XeroCliError {
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth {
            message: msg.into(),
        }
    }

    pub fn api(status: u16, msg: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: msg.into(),
        }
    }

    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config {
            message: msg.into(),
        }
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation {
            message: msg.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_auth() {
        let err = XeroCliError::auth("token expired");
        assert_eq!(err.to_string(), "Authentication failed: token expired");
    }

    #[test]
    fn error_display_api() {
        let err = XeroCliError::api(404, "Invoice not found");
        assert_eq!(err.to_string(), "API error (404): Invoice not found");
    }

    #[test]
    fn error_display_rate_limited() {
        let err = XeroCliError::RateLimited {
            retry_after_secs: 30,
        };
        assert!(err.to_string().contains("30s"));
    }

    #[test]
    fn error_has_diagnostic_code() {
        use miette::Diagnostic;
        let err = XeroCliError::auth("test");
        assert!(err.code().is_some());
    }
}
