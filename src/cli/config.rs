use crate::cli::GlobalArgs;
use crate::config::AppConfig;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize configuration file
    Init,
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Config key (e.g. "auth.client_id", "default.output_format")
        key: String,
        /// Config value
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
