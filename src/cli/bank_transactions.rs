use crate::api::endpoints::bank_transactions::{self, BankTransactionFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::bank_transaction::BankTransaction;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BankTransactionCommands {
    /// List bank transactions
    ///
    /// Retrieve bank transactions (spend or receive money) with optional
    /// filtering by bank account, date range, and custom where clause.
    /// Results include SPEND and RECEIVE transaction types.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transactions list
  xero bank-transactions list --account 7d05a53d-613d-4eb2-a2fc-dcb6adb80b80
  xero bank-transactions list --from 2024-01-01
  xero bank-transactions list --where 'Type==\"SPEND\"' --output json
  xero bank-transactions list --account 7d05a53d-... --from 2024-06-01 --where 'Status==\"AUTHORISED\"'")]
    List {
        /// Filter by bank account ID (UUID) to show only transactions for that account
        #[arg(long)]
        account: Option<String>,
        /// Filter from date in YYYY-MM-DD format (inclusive)
        #[arg(long)]
        from: Option<String>,
        /// Custom Xero where clause filter expression
        ///
        /// Uses Xero's filter syntax: Field==Value, Field!=Value,
        /// Field.Contains("value"), Field.StartsWith("value").
        /// Multiple conditions joined with &&.
        /// Example: --where 'Type=="SPEND"&&Status=="AUTHORISED"'
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },

    /// Get a specific bank transaction
    ///
    /// Retrieve full details for a single bank transaction by its UUID,
    /// including line items, contact, and bank account information.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transactions get b5e8f1a2-3c4d-5678-9012-abcdef345678
  xero bank-transactions get b5e8f1a2-... --output json")]
    Get {
        /// Bank transaction ID (UUID)
        id: String,
    },

    /// Create a bank transaction
    ///
    /// Create a new bank transaction from a JSON file. The file must
    /// contain valid Xero BankTransaction JSON, including Type (SPEND
    /// or RECEIVE), Contact, BankAccount, and at least one LineItem.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transactions create --file spend.json
  xero bank-transactions create --file receive.json --output json")]
    Create {
        /// Path to a JSON file containing the bank transaction payload
        #[arg(long)]
        file: String,
    },

    /// Delete a bank transaction
    ///
    /// Delete a bank transaction by setting its status to DELETED.
    /// The record is not removed; it remains visible with a DELETED status.
    /// Only AUTHORISED transactions can be deleted.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transactions delete b5e8f1a2-3c4d-5678-9012-abcdef345678")]
    Delete {
        /// Bank transaction ID (UUID) to delete
        id: String,
    },

    /// View bank transaction history
    ///
    /// Retrieve the audit history for a bank transaction, showing all
    /// status changes, modifications, and user actions.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transactions history b5e8f1a2-3c4d-5678-9012-abcdef345678
  xero bank-transactions history b5e8f1a2-... --output json")]
    History {
        /// Bank transaction ID (UUID)
        id: String,
    },
}

impl Tabular for BankTransaction {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Type".to_string(),
            "Contact".to_string(),
            "Account".to_string(),
            "Total".to_string(),
            "Date".to_string(),
            "Status".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.bank_transaction_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.transaction_type
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
            self.contact
                .as_ref()
                .and_then(|c| c.name.clone())
                .unwrap_or_default(),
            self.bank_account
                .as_ref()
                .and_then(|a| a.code.clone())
                .unwrap_or_default(),
            self.total.map(|t| t.to_string()).unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
            self.status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: BankTransactionCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        BankTransactionCommands::List {
            account,
            from,
            where_clause,
        } => {
            let filters = BankTransactionFilters {
                account_id: account,
                date_from: from,
                where_clause,
                ..Default::default()
            };
            let list = bank_transactions::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        BankTransactionCommands::Get { id } => {
            let bt = bank_transactions::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&bt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        BankTransactionCommands::Create { file } => {
            let content = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let bt = bank_transactions::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&bt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        BankTransactionCommands::Delete { id } => {
            let bt = bank_transactions::delete(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Bank transaction deleted successfully.");
            let rendered = output::render_single(&bt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        BankTransactionCommands::History { id } => {
            let records = bank_transactions::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
