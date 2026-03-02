use crate::api::endpoints::tracking_categories;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::tracking_category::{TrackingCategory, TrackingOption};
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TrackingCategoryCommands {
    /// List tracking categories
    ///
    /// Retrieve all tracking categories and their options for the connected
    /// Xero organisation. Tracking categories let you tag transactions with
    /// additional dimensions such as region, department, or project.
    #[command(after_long_help = "\
EXAMPLES:
  xero tracking-categories list
  xero tracking-categories list --output json
  xero tracking-categories list --compact")]
    List,

    /// Get a specific tracking category
    ///
    /// Retrieve full details for a single tracking category by its UUID,
    /// including all of its available options.
    #[command(after_long_help = "\
EXAMPLES:
  xero tracking-categories get b2e5f3a1-7d4c-4e8a-9f12-3c6d8e9a0b1f
  xero tracking-categories get b2e5f3a1-... --output json")]
    Get {
        /// Tracking category ID (UUID)
        id: String,
    },

    /// Create a tracking category
    ///
    /// Create a new tracking category with the given name. Xero allows a
    /// maximum of two active tracking categories at any time.
    #[command(after_long_help = "\
EXAMPLES:
  xero tracking-categories create --name \"Region\"
  xero tracking-categories create --name \"Department\" --output json")]
    Create {
        /// Name for the new tracking category
        #[arg(long)]
        name: String,
    },

    /// Update a tracking category
    ///
    /// Rename an existing tracking category. This does not affect the
    /// options within the category.
    #[command(after_long_help = "\
EXAMPLES:
  xero tracking-categories update b2e5f3a1-... --name \"Cost Centre\"
  xero tracking-categories update b2e5f3a1-... --name \"Business Unit\" --output json")]
    Update {
        /// Tracking category ID (UUID) to update
        id: String,
        /// New name for the tracking category
        #[arg(long)]
        name: String,
    },

    /// Add an option to a tracking category
    ///
    /// Add a new selectable option to an existing tracking category.
    /// For example, add "North" and "South" options to a "Region" category.
    #[command(after_long_help = "\
EXAMPLES:
  xero tracking-categories add-option b2e5f3a1-... --name \"North\"
  xero tracking-categories add-option b2e5f3a1-... --name \"South\" --output json")]
    AddOption {
        /// Tracking category ID (UUID) to add the option to
        id: String,
        /// Name for the new option
        #[arg(long)]
        name: String,
    },

    /// Update an option in a tracking category
    ///
    /// Rename an existing option within a tracking category.
    #[command(after_long_help = "\
EXAMPLES:
  xero tracking-categories update-option b2e5f3a1-... --option-id c4d6e8f0-... --name \"North-East\"
  xero tracking-categories update-option b2e5f3a1-... --option-id c4d6e8f0-... --name \"APAC\" --output json")]
    UpdateOption {
        /// Tracking category ID (UUID) containing the option
        id: String,
        /// Option ID (UUID) to update
        #[arg(long)]
        option_id: String,
        /// New name for the option
        #[arg(long)]
        name: String,
    },

    /// Remove an option from a tracking category
    ///
    /// Delete an option from a tracking category. Existing transactions
    /// that use this option will retain the value, but it will no longer
    /// be selectable for new transactions.
    #[command(after_long_help = "\
EXAMPLES:
  xero tracking-categories remove-option b2e5f3a1-... --option-id c4d6e8f0-...")]
    RemoveOption {
        /// Tracking category ID (UUID) containing the option
        id: String,
        /// Option ID (UUID) to remove
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
