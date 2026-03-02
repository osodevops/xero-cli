use crate::cli::GlobalArgs;
use crate::config::AppConfig;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize configuration file.
    ///
    /// Creates a new configuration file with sensible defaults at the default
    /// location (~/.config/xero-cli/config.toml) or at the path specified by
    /// the `--config` global option. If the file already exists, the command
    /// exits without overwriting it.
    #[command(after_long_help = r#"EXAMPLES:
  # Create a default config file
  xero config init

  # Create a config file at a custom path
  xero --config ./my-config.toml config init"#)]
    Init,

    /// Show current configuration.
    ///
    /// Prints the fully resolved configuration as TOML to stdout. This includes
    /// values loaded from the config file as well as any defaults applied by the
    /// application. Useful for verifying that your settings are correct before
    /// making API calls.
    #[command(after_long_help = r#"EXAMPLES:
  # Display the current configuration
  xero config show

  # Display configuration from a specific file
  xero --config ./my-config.toml config show"#)]
    Show,

    /// Set a configuration value.
    ///
    /// Updates a single key in the configuration file using dotted notation
    /// (section.field). The config file is created if it does not already exist.
    /// Changes are written immediately and take effect on the next command
    /// invocation.
    #[command(after_long_help = r#"EXAMPLES:
  # Set the OAuth2 client ID
  xero config set auth.client_id YOUR_CLIENT_ID

  # Set the default output format to JSON
  xero config set default.output_format json

  # Set the tenant ID for multi-org setups
  xero config set auth.tenant_id YOUR_TENANT_ID"#)]
    Set {
        /// Config key in dotted notation (e.g. "auth.client_id", "default.output_format").
        ///
        /// The key must contain exactly one dot separating the TOML section name
        /// from the field name.
        key: String,
        /// The value to assign to the key.
        value: String,
    },
}

pub async fn execute(command: ConfigCommands, global: &GlobalArgs) -> miette::Result<()> {
    match command {
        ConfigCommands::Init => {
            let config_path = global
                .config_path
                .clone()
                .or_else(|| {
                    crate::config::default_config_path()
                        .ok()
                        .map(|p| p.to_string_lossy().to_string())
                })
                .ok_or_else(|| miette::miette!("Could not determine config path"))?;

            let path = std::path::Path::new(&config_path);
            if path.exists() {
                eprintln!("Config file already exists at: {config_path}");
                return Ok(());
            }

            let config = AppConfig::load(Some(&config_path)).map_err(|e| miette::miette!("{e}"))?;
            config.save().map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Config file created at: {config_path}");
        }

        ConfigCommands::Show => {
            let config = AppConfig::load(global.config_path.as_deref())
                .map_err(|e| miette::miette!("{e}"))?;
            let content = toml::to_string_pretty(&config.config_file)
                .map_err(|e| miette::miette!("Failed to serialize config: {e}"))?;
            println!("{content}");
        }

        ConfigCommands::Set { key, value } => {
            let config_path = global
                .config_path
                .clone()
                .or_else(|| {
                    crate::config::default_config_path()
                        .ok()
                        .map(|p| p.to_string_lossy().to_string())
                })
                .ok_or_else(|| miette::miette!("Could not determine config path"))?;

            let path = std::path::Path::new(&config_path);
            let content = if path.exists() {
                std::fs::read_to_string(path)
                    .map_err(|e| miette::miette!("Failed to read config: {e}"))?
            } else {
                String::new()
            };

            let mut doc = content
                .parse::<toml_edit::DocumentMut>()
                .map_err(|e| miette::miette!("Failed to parse config: {e}"))?;

            let parts: Vec<&str> = key.split('.').collect();
            match parts.as_slice() {
                [section, field] => {
                    doc[section][field] = toml_edit::value(&value);
                }
                _ => {
                    return Err(miette::miette!(
                        "Key format: section.field (e.g. auth.client_id)"
                    ));
                }
            }

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| miette::miette!("Failed to create config directory: {e}"))?;
            }
            std::fs::write(path, doc.to_string())
                .map_err(|e| miette::miette!("Failed to write config: {e}"))?;
            eprintln!("Set {key} = {value}");
        }
    }

    Ok(())
}
