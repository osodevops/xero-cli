use crate::api::endpoints::linked_transactions;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::linked_transaction::LinkedTransaction;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum LinkedTransactionCommands {
    /// List linked transactions
    List,
    /// Get a specific linked transaction
    Get { id: String },
    /// Create a linked transaction
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a linked transaction
    Update {
        id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a linked transaction
    Delete { id: String },
}

impl Tabular for LinkedTransaction {
    fn headers() -> Vec<String> {
        vec!["ID".into(), "Type".into(), "Status".into(), "Source".into()]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.linked_transaction_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.transaction_type.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.source_transaction_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
        ]
    }
}

pub async fn execute(
    command: LinkedTransactionCommands,
    global: &GlobalArgs,
) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        LinkedTransactionCommands::List => {
            let list = linked_transactions::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        LinkedTransactionCommands::Get { id } => {
            let lt = linked_transactions::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&lt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        LinkedTransactionCommands::Create { file } => {
            let c = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let lt = linked_transactions::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&lt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        LinkedTransactionCommands::Update { id, file } => {
            let c = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let lt = linked_transactions::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&lt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        LinkedTransactionCommands::Delete { id } => {
            linked_transactions::delete(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Linked transaction deleted successfully.");
        }
    }
    Ok(())
}
