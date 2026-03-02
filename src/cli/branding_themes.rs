use crate::api::endpoints::branding_themes;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::branding_theme::BrandingTheme;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BrandingThemeCommands {
    /// List branding themes
    List,
    /// Get a specific branding theme
    Get { id: String },
}

impl Tabular for BrandingTheme {
    fn headers() -> Vec<String> {
        vec!["ID".to_string(), "Name".to_string(), "Type".to_string()]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.branding_theme_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.name.clone().unwrap_or_default(),
            self.theme_type.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: BrandingThemeCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        BrandingThemeCommands::List => {
            let list = branding_themes::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        BrandingThemeCommands::Get { id } => {
            let bt = branding_themes::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&bt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
