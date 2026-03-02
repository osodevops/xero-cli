use crate::api::endpoints::accounts::{self, AccountFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::account::Account;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum AccountCommands {
    /// List accounts
    ///
    /// Retrieve accounts from the chart of accounts with optional type and class filters.
    #[command(after_long_help = "\
EXAMPLES:
  xero accounts list
  xero accounts list --type BANK
  xero accounts list --class REVENUE --order \"Code ASC\"
  xero accounts list --where 'Status==\"ACTIVE\"' --output json")]
    List {
        /// Filter by account type: BANK, CURRENT, CURRLIAB, DEPRECIATN, DIRECTCOSTS,
        /// EQUITY, EXPENSE, FIXED, INVENTORY, LIABILITY, NONCURRENT, OTHERINCOME,
        /// OVERHEADS, PREPAYMENT, REVENUE, SALES, TERMLIAB, PAYGLIABILITY,
        /// SUPERANNUATIONEXPENSE, SUPERANNUATIONLIABILITY, WAGESEXPENSE
        #[arg(long, name = "type")]
        account_type: Option<String>,
        /// Filter by account class: ASSET, EQUITY, EXPENSE, LIABILITY, REVENUE
        #[arg(long)]
        class: Option<String>,
        /// Custom Xero where clause filter expression
        #[arg(long, name = "where")]
        where_clause: Option<String>,
        /// Order by field and direction (e.g. "Code ASC", "Name DESC")
        #[arg(long)]
        order: Option<String>,
    },

    /// Get a specific account
    ///
    /// Retrieve full details for a single account by its UUID.
    #[command(after_long_help = "\
EXAMPLES:
  xero accounts get 7d05a53d-613d-4eb2-a2fc-dcb6adb80b80")]
    Get {
        /// Account ID (UUID)
        id: String,
    },

    /// Create an account
    ///
    /// Add a new account to the chart of accounts.
    /// Name, code, and type are required.
    #[command(after_long_help = "\
EXAMPLES:
  xero accounts create --name \"Office Supplies\" --code 429 --type EXPENSE
  xero accounts create --name \"Sales Income\" --code 200 --type REVENUE --tax-type OUTPUT")]
    Create {
        /// Account name
        #[arg(long)]
        name: String,
        /// Account code (must be unique within the organisation)
        #[arg(long)]
        code: String,
        /// Account type (see `xero accounts list --help` for valid types)
        #[arg(long, name = "type")]
        account_type: String,
        /// Account description
        #[arg(long)]
        description: Option<String>,
        /// Tax type code (e.g. OUTPUT, INPUT, NONE)
        #[arg(long)]
        tax_type: Option<String>,
    },

    /// Archive an account
    ///
    /// Archive an account so it no longer appears in active lists.
    /// Archived accounts can still be viewed but cannot be used in new transactions.
    #[command(after_long_help = "\
EXAMPLES:
  xero accounts archive 7d05a53d-613d-4eb2-a2fc-dcb6adb80b80")]
    Archive {
        /// Account ID (UUID)
        id: String,
    },
}

impl Tabular for Account {
    fn headers() -> Vec<String> {
        vec![
            "Code".to_string(),
            "Name".to_string(),
            "Type".to_string(),
            "Class".to_string(),
            "Status".to_string(),
            "Tax Type".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.code.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.account_type
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
            self.class
                .as_ref()
                .map(|c| c.to_string())
                .unwrap_or_default(),
            self.status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            self.tax_type.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: AccountCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        AccountCommands::List {
            account_type,
            class,
            where_clause,
            order,
        } => {
            let filters = AccountFilters {
                account_type,
                class,
                where_clause,
                order,
            };
            let accounts_list = accounts::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&accounts_list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        AccountCommands::Get { id } => {
            let account = accounts::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&account, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        AccountCommands::Create {
            name,
            code,
            account_type,
            description,
            tax_type,
        } => {
            let mut body = serde_json::json!({
                "Name": name,
                "Code": code,
                "Type": account_type,
            });
            if let Some(desc) = description {
                body["Description"] = serde_json::Value::String(desc);
            }
            if let Some(tax) = tax_type {
                body["TaxType"] = serde_json::Value::String(tax);
            }

            let account = accounts::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&account, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        AccountCommands::Archive { id } => {
            let account = accounts::archive(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Account archived successfully.");
            let rendered = output::render_single(&account, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
