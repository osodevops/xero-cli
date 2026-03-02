use crate::api::endpoints::accounts::{self, AccountFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::account::Account;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum AccountCommands {
    /// List accounts
    List {
        /// Filter by type (BANK, REVENUE, EXPENSE, etc.)
        #[arg(long, name = "type")]
        account_type: Option<String>,
        /// Filter by class (ASSET, EQUITY, EXPENSE, LIABILITY, REVENUE)
        #[arg(long)]
        class: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
        /// Order by
        #[arg(long)]
        order: Option<String>,
    },
    /// Get a specific account
    Get {
        /// Account ID
        id: String,
    },
    /// Create an account
    Create {
        /// Account name
        #[arg(long)]
        name: String,
        /// Account code
        #[arg(long)]
        code: String,
        /// Account type
        #[arg(long, name = "type")]
        account_type: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Tax type
        #[arg(long)]
        tax_type: Option<String>,
    },
    /// Archive an account
    Archive {
        /// Account ID
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
