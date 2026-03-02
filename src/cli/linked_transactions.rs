use crate::api::endpoints::linked_transactions;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::linked_transaction::LinkedTransaction;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum LinkedTransactionCommands {
    /// List linked transactions
    ///
    /// Retrieve all linked transactions in the organisation.
    /// Linked transactions connect billable expenses on purchase transactions
    /// (e.g. bills) to sales invoices so costs can be passed on to customers.
    #[command(after_long_help = "\
EXAMPLES:
  xero linked-transactions list
  xero linked-transactions list --output json")]
    List,

    /// Get a specific linked transaction
    ///
    /// Retrieve full details for a single linked transaction by its UUID,
    /// including the source and target transaction references.
    #[command(after_long_help = "\
EXAMPLES:
  xero linked-transactions get a1b2c3d4-e5f6-7890-abcd-ef1234567890
  xero linked-transactions get a1b2c3d4-e5f6-7890-abcd-ef1234567890 --output json")]
    Get {
        /// Linked transaction ID (UUID)
        id: String,
    },

    /// Create a linked transaction
    ///
    /// Create a new linked transaction from a JSON file.
    /// The file must contain a valid payload specifying the source transaction,
    /// source line item, and the target contact to bill.
    #[command(after_long_help = "\
EXAMPLES:
  xero linked-transactions create --file link.json
  xero linked-transactions create --file link.json --output json")]
    Create {
        /// Path to JSON file containing the linked transaction payload
        #[arg(long)]
        file: String,
    },

    /// Update a linked transaction
    ///
    /// Update an existing linked transaction from a JSON file.
    /// The file must contain the fields to modify, such as the target
    /// transaction or contact assignment.
    #[command(after_long_help = "\
EXAMPLES:
  xero linked-transactions update a1b2c3d4-... --file updated-link.json
  xero linked-transactions update a1b2c3d4-... --file updated-link.json --output json")]
    Update {
        /// Linked transaction ID (UUID)
        id: String,
        /// Path to JSON file with updated linked transaction data
        #[arg(long)]
        file: String,
    },

    /// Delete a linked transaction
    ///
    /// Permanently delete a linked transaction by its UUID.
    /// This removes the link between the source and target transactions
    /// but does not affect the underlying transactions themselves.
    #[command(after_long_help = "\
EXAMPLES:
  xero linked-transactions delete a1b2c3d4-e5f6-7890-abcd-ef1234567890")]
    Delete {
        /// Linked transaction ID (UUID)
        id: String,
    },
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
