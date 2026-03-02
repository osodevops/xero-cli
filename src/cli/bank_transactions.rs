use crate::api::endpoints::bank_transactions::{self, BankTransactionFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::bank_transaction::BankTransaction;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BankTransactionCommands {
    /// List bank transactions
    List {
        /// Filter by bank account ID
        #[arg(long)]
        account: Option<String>,
        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },
    /// Get a specific bank transaction
    Get {
        /// Bank transaction ID
        id: String,
    },
    /// Create a bank transaction
    Create {
        /// JSON file with transaction data
        #[arg(long)]
        file: String,
    },
    /// Delete a bank transaction
    Delete {
        /// Bank transaction ID
        id: String,
    },
    /// View bank transaction history
    History {
        /// Bank transaction ID
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
