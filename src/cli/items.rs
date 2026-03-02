use crate::api::endpoints::items::{self, ItemFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::item::Item;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ItemCommands {
    /// List items
    List {
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },
    /// Get a specific item
    Get {
        /// Item ID
        id: String,
    },
    /// Create an item
    Create {
        /// Item code
        #[arg(long)]
        code: String,
        /// Item name
        #[arg(long)]
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Sale unit price
        #[arg(long)]
        sale_price: Option<String>,
        /// Sale account code
        #[arg(long)]
        sale_account: Option<String>,
        /// Purchase unit price
        #[arg(long)]
        purchase_price: Option<String>,
        /// Purchase account code
        #[arg(long)]
        purchase_account: Option<String>,
    },
    /// Update an item
    Update {
        /// Item ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New sale price
        #[arg(long)]
        sale_price: Option<String>,
        /// New purchase price
        #[arg(long)]
        purchase_price: Option<String>,
    },
    /// Delete an item
    Delete {
        /// Item ID
        id: String,
    },
    /// View item history
    History {
        /// Item ID
        id: String,
    },
}

impl Tabular for Item {
    fn headers() -> Vec<String> {
        vec![
            "Code".to_string(),
            "Name".to_string(),
            "Sale Price".to_string(),
            "Purchase Price".to_string(),
            "Description".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.code.clone().unwrap_or_default(),
            self.name.clone().unwrap_or_default(),
            self.sales_details
                .as_ref()
                .and_then(|s| s.unit_price.map(|p| p.to_string()))
                .unwrap_or_default(),
            self.purchase_details
                .as_ref()
                .and_then(|p| p.unit_price.map(|p| p.to_string()))
                .unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: ItemCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        ItemCommands::List { where_clause } => {
            let filters = ItemFilters {
                where_clause,
                ..Default::default()
            };
            let items_list = items::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&items_list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        ItemCommands::Get { id } => {
            let item = items::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&item, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        ItemCommands::Create {
            code,
            name,
            description,
            sale_price,
            sale_account,
            purchase_price,
            purchase_account,
        } => {
            let mut body = serde_json::json!({
                "Code": code,
                "Name": name,
            });
            if let Some(desc) = description {
                body["Description"] = serde_json::Value::String(desc);
            }
            if sale_price.is_some() || sale_account.is_some() {
                let mut sales = serde_json::json!({});
                if let Some(price) = sale_price {
                    sales["UnitPrice"] = serde_json::Value::String(price);
                }
                if let Some(acct) = sale_account {
                    sales["AccountCode"] = serde_json::Value::String(acct);
                }
                body["SalesDetails"] = sales;
            }
            if purchase_price.is_some() || purchase_account.is_some() {
                let mut purchase = serde_json::json!({});
                if let Some(price) = purchase_price {
                    purchase["UnitPrice"] = serde_json::Value::String(price);
                }
                if let Some(acct) = purchase_account {
                    purchase["AccountCode"] = serde_json::Value::String(acct);
                }
                body["PurchaseDetails"] = purchase;
            }

            let item = items::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&item, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        ItemCommands::Update {
            id,
            name,
            sale_price,
            purchase_price,
        } => {
            let mut body = serde_json::json!({});
            if let Some(n) = name {
                body["Name"] = serde_json::Value::String(n);
            }
            if let Some(price) = sale_price {
                body["SalesDetails"] = serde_json::json!({"UnitPrice": price});
            }
            if let Some(price) = purchase_price {
                body["PurchaseDetails"] = serde_json::json!({"UnitPrice": price});
            }

            let item = items::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&item, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        ItemCommands::Delete { id } => {
            items::delete(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Item deleted successfully.");
        }

        ItemCommands::History { id } => {
            let records = items::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
