use crate::auth::token_store::TokenStore;
use crate::cli::GlobalArgs;
use crate::config::AppConfig;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Interactive PKCE login
    ///
    /// Opens a browser for the OAuth2 Authorization Code + PKCE flow.
    /// A local HTTP server listens on the callback port to receive the auth code.
    /// Tokens are stored securely in the OS keychain.
    #[command(after_long_help = "\
EXAMPLES:
  xero auth login
  xero auth login --port 9090
  xero auth login --scopes \"openid,accounting.invoices.read\"")]
    Login {
        /// Custom callback port for the local HTTP server
        #[arg(long, default_value = "8080")]
        port: u16,
        /// Comma-separated OAuth scopes (overrides config file scopes)
        ///
        /// Common scopes: openid, profile, email, accounting.invoices,
        /// accounting.invoices.read, accounting.payments,
        /// accounting.payments.read, accounting.banktransactions.read,
        /// accounting.contacts, accounting.contacts.read,
        /// accounting.settings, accounting.settings.read,
        /// accounting.reports.profitandloss.read,
        /// accounting.reports.balancesheet.read,
        /// accounting.budgets.read
        #[arg(long)]
        scopes: Option<String>,
    },

    /// Show current auth status
    ///
    /// Displays whether authentication is active or expired, the tenant ID,
    /// token expiry time, time remaining, and configured scopes.
    Status,

    /// Force token refresh
    ///
    /// Uses the stored refresh token to obtain a new access token.
    /// Useful when the access token has expired but the refresh token is still valid.
    Refresh,

    /// Clear stored tokens
    ///
    /// Removes all stored OAuth tokens from the OS keychain.
    /// You will need to run `xero auth login` again after logging out.
    Logout,

    /// Configure client credentials (M2M)
    ///
    /// Set up machine-to-machine authentication using client credentials flow.
    /// This is used for server-to-server integrations without user interaction.
    /// Requires a Xero app configured with the "Client Credentials" grant type.
    #[command(after_long_help = "\
EXAMPLES:
  xero auth setup-m2m --client-id YOUR_ID --client-secret YOUR_SECRET")]
    SetupM2m {
        /// OAuth2 client ID from your Xero app
        #[arg(long, env = "XERO_CLIENT_ID")]
        client_id: String,
        /// OAuth2 client secret from your Xero app
        #[arg(long, env = "XERO_CLIENT_SECRET")]
        client_secret: String,
    },

    /// Manage OAuth scopes
    ///
    /// View, add, or apply preset scope configurations.
    /// Without a subcommand, lists currently configured scopes.
    /// Changes take effect after re-authenticating with `xero auth login`.
    Scopes {
        #[command(subcommand)]
        command: Option<ScopeCommands>,
    },
}

#[derive(Subcommand)]
pub enum ScopeCommands {
    /// Add a scope
    ///
    /// Adds a single OAuth scope to the configuration.
    /// Re-authenticate with `xero auth login` for the change to take effect.
    #[command(after_long_help = "\
EXAMPLES:
  xero auth scopes add accounting.invoices.read
  xero auth scopes add accounting.contacts")]
    Add {
        /// OAuth scope to add (e.g. accounting.invoices.read)
        scope: String,
    },

    /// Apply a scope preset
    ///
    /// Replaces all configured scopes with a predefined set.
    /// Available presets: read-only, bookkeeper, full-access, reports-only.
    #[command(after_long_help = "\
EXAMPLES:
  xero auth scopes preset read-only
  xero auth scopes preset full-access")]
    Preset {
        /// Preset name: read-only, bookkeeper, full-access, reports-only
        name: String,
    },
}

pub async fn execute(command: AuthCommands, global: &GlobalArgs) -> miette::Result<()> {
    let config =
        AppConfig::load(global.config_path.as_deref()).map_err(|e| miette::miette!("{e}"))?;
    let config_dir = crate::config::default_config_dir().map_err(|e| miette::miette!("{e}"))?;
    let store = TokenStore::new(config_dir);

    match command {
        AuthCommands::Login { port, scopes } => {
            let client_id = config
                .config_file
                .auth
                .client_id
                .as_deref()
                .map(|s| s.to_string())
                .or_else(|| std::env::var("XERO_CLIENT_ID").ok())
                .ok_or_else(|| {
                    miette::miette!(
                        "No client_id configured. Set it in config.toml or XERO_CLIENT_ID env var"
                    )
                })?;

            let client_secret = std::env::var("XERO_CLIENT_SECRET").ok();

            let scope_list: Vec<String> = scopes
                .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or(config.config_file.auth.scopes.clone());

            let tokens =
                crate::auth::pkce::login(&client_id, client_secret.as_deref(), &scope_list, port)
                    .await
                    .map_err(|e| miette::miette!("{e}"))?;

            store.save(&tokens).map_err(|e| miette::miette!("{e}"))?;

            eprintln!("Login successful!");
            if let Some(ref tid) = tokens.tenant_id {
                eprintln!("Tenant ID: {tid}");
            }
            eprintln!(
                "Token expires at: {}",
                tokens.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }

        AuthCommands::Status => match store.load() {
            Ok(tokens) => {
                let remaining = tokens.time_remaining();
                let status = if tokens.is_expired() {
                    "EXPIRED"
                } else {
                    "ACTIVE"
                };
                eprintln!("Auth status: {status}");
                if let Some(ref tid) = tokens.tenant_id {
                    eprintln!("Tenant ID: {tid}");
                }
                eprintln!(
                    "Expires at: {}",
                    tokens.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
                );
                if !tokens.is_expired() {
                    eprintln!(
                        "Time remaining: {}m {}s",
                        remaining.num_minutes(),
                        remaining.num_seconds() % 60
                    );
                }
                if !tokens.scopes.is_empty() {
                    eprintln!("Scopes: {}", tokens.scopes.join(", "));
                }
            }
            Err(_) => {
                eprintln!("Not authenticated. Run `xero auth login` to authenticate.");
            }
        },

        AuthCommands::Refresh => {
            let tokens = store.load().map_err(|e| miette::miette!("{e}"))?;
            let client_id = config
                .config_file
                .auth
                .client_id
                .as_deref()
                .map(|s| s.to_string())
                .or_else(|| std::env::var("XERO_CLIENT_ID").ok())
                .ok_or_else(|| miette::miette!("No client_id configured"))?;

            let refresh_token = tokens
                .refresh_token
                .as_deref()
                .ok_or_else(|| miette::miette!("No refresh token available"))?;

            let client_secret = std::env::var("XERO_CLIENT_SECRET").ok();
            let new_tokens = crate::auth::refresh::refresh_token(
                &client_id,
                client_secret.as_deref(),
                refresh_token,
            )
            .await
            .map_err(|e| miette::miette!("{e}"))?;

            store
                .save(&new_tokens)
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Token refreshed successfully!");
            eprintln!(
                "New expiry: {}",
                new_tokens.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }

        AuthCommands::Logout => {
            store.clear().map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Logged out. Stored tokens cleared.");
        }

        AuthCommands::SetupM2m {
            client_id,
            client_secret,
        } => {
            let scopes = config.config_file.auth.scopes.clone();
            let tokens =
                crate::auth::client_credentials::authenticate(&client_id, &client_secret, &scopes)
                    .await
                    .map_err(|e| miette::miette!("{e}"))?;

            store.save(&tokens).map_err(|e| miette::miette!("{e}"))?;
            eprintln!("M2M authentication configured and tokens stored.");

            // Save client_id to config
            let mut config = config;
            config.config_file.auth.client_id = Some(client_id);
            config.save().map_err(|e| miette::miette!("{e}"))?;
        }

        AuthCommands::Scopes { command } => match command {
            None => {
                let scopes = &config.config_file.auth.scopes;
                if scopes.is_empty() {
                    eprintln!("No scopes configured.");
                } else {
                    for scope in scopes {
                        println!("{scope}");
                    }
                }
            }
            Some(ScopeCommands::Add { scope }) => {
                let mut config = config;
                if !config.config_file.auth.scopes.contains(&scope) {
                    config.config_file.auth.scopes.push(scope.clone());
                    config.save().map_err(|e| miette::miette!("{e}"))?;
                    eprintln!("Added scope: {scope}");
                    eprintln!("Re-authenticate with `xero auth login` for changes to take effect.");
                } else {
                    eprintln!("Scope already configured: {scope}");
                }
            }
            Some(ScopeCommands::Preset { name }) => {
                let scopes = crate::config::profiles::scope_preset(&name).ok_or_else(|| {
                        miette::miette!(
                            "Unknown preset: {name}. Available: read-only, bookkeeper, full-access, reports-only"
                        )
                    })?;
                let mut config = config;
                config.config_file.auth.scopes = scopes;
                config.save().map_err(|e| miette::miette!("{e}"))?;
                eprintln!("Applied scope preset: {name}");
                eprintln!("Re-authenticate with `xero auth login` for changes to take effect.");
            }
        },
    }

    Ok(())
}
