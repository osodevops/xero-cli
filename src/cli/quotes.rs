use crate::api::endpoints::quotes::{self, QuoteFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::quote::Quote;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum QuoteCommands {
    /// List quotes
    List {
        /// Filter by status (DRAFT, SENT, ACCEPTED, DECLINED, INVOICED)
        #[arg(long)]
        status: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },
    /// Get a specific quote
    Get {
        /// Quote ID
        id: String,
    },
    /// Create a quote
    Create {
        /// JSON file with quote data
        #[arg(long)]
        file: String,
    },
    /// Update a quote
    Update {
        /// Quote ID
        id: String,
        /// New expiry date (YYYY-MM-DD)
        #[arg(long)]
        expiry_date: Option<String>,
        /// New status
        #[arg(long)]
        status: Option<String>,
        /// JSON file with update data
        #[arg(long)]
        file: Option<String>,
    },
}

impl Tabular for Quote {
    fn headers() -> Vec<String> {
        vec![
            "Quote #".to_string(),
            "Contact".to_string(),
            "Status".to_string(),
            "Total".to_string(),
            "Date".to_string(),
            "Expiry".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.quote_number.clone().unwrap_or_default(),
            self.contact
                .as_ref()
                .and_then(|c| c.name.clone())
                .unwrap_or_default(),
            self.status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            self.total.map(|t| t.to_string()).unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
            self.expiry_date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: QuoteCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        QuoteCommands::List {
            status,
            where_clause,
        } => {
            let filters = QuoteFilters {
                status,
                where_clause,
                ..Default::default()
            };
            let list = quotes::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        QuoteCommands::Get { id } => {
            let quote = quotes::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&quote, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        QuoteCommands::Create { file } => {
            let content = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let quote = quotes::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&quote, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        QuoteCommands::Update {
            id,
            expiry_date,
            status,
            file,
        } => {
            let body = if let Some(file_path) = file {
                let content = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else {
                let mut body = serde_json::json!({});
                if let Some(date) = expiry_date {
                    body["ExpiryDate"] = serde_json::Value::String(date);
                }
                if let Some(s) = status {
                    body["Status"] = serde_json::Value::String(s);
                }
                body
            };

            let quote = quotes::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&quote, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
