use crate::api::endpoints::tracking_categories;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::tracking_category::{TrackingCategory, TrackingOption};
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TrackingCategoryCommands {
    /// List tracking categories
    List,
    /// Get a specific tracking category
    Get {
        /// Tracking category ID
        id: String,
    },
    /// Create a tracking category
    Create {
        /// Category name
        #[arg(long)]
        name: String,
    },
    /// Update a tracking category
    Update {
        /// Tracking category ID
        id: String,
        /// New category name
        #[arg(long)]
        name: String,
    },
    /// Add an option to a tracking category
    AddOption {
        /// Tracking category ID
        id: String,
        /// Option name
        #[arg(long)]
        name: String,
    },
    /// Update an option in a tracking category
    UpdateOption {
        /// Tracking category ID
        id: String,
        /// Option ID to update
        #[arg(long)]
        option_id: String,
        /// New option name
        #[arg(long)]
        name: String,
    },
    /// Remove an option from a tracking category
    RemoveOption {
        /// Tracking category ID
        id: String,
        /// Option ID to remove
        #[arg(long)]
        option_id: String,
    },
}

impl Tabular for TrackingCategory {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Name".to_string(),
            "Status".to_string(),
            "Options".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.tracking_category_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.name.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.options
                .as_ref()
                .map(|opts| opts.len().to_string())
                .unwrap_or_else(|| "0".to_string()),
        ]
    }
}

impl Tabular for TrackingOption {
    fn headers() -> Vec<String> {
        vec![
            "Option ID".to_string(),
            "Name".to_string(),
            "Status".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.tracking_option_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.name.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: TrackingCategoryCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        TrackingCategoryCommands::List => {
            let list = tracking_categories::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        TrackingCategoryCommands::Get { id } => {
            let tc = tracking_categories::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&tc, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        TrackingCategoryCommands::Create { name } => {
            let data = serde_json::json!({"Name": name});
            let tc = tracking_categories::create(&client, &data)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&tc, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        TrackingCategoryCommands::Update { id, name } => {
            let data = serde_json::json!({"Name": name});
            let tc = tracking_categories::update(&client, &id, &data)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&tc, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        TrackingCategoryCommands::AddOption { id, name } => {
            let data = serde_json::json!({"Name": name});
            let opt = tracking_categories::add_option(&client, &id, &data)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&opt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        TrackingCategoryCommands::UpdateOption {
            id,
            option_id,
            name,
        } => {
            let data = serde_json::json!({"Name": name});
            let opt = tracking_categories::update_option(&client, &id, &option_id, &data)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&opt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        TrackingCategoryCommands::RemoveOption { id, option_id } => {
            tracking_categories::delete_option(&client, &id, &option_id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            if !global.quiet {
                println!("Option {option_id} removed from tracking category {id}");
            }
        }
    }

    Ok(())
}
