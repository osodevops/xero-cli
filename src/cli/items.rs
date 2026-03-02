use crate::api::endpoints::items::{self, ItemFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::item::Item;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ItemCommands {
    /// List items
    ///
    /// Retrieve items from the Xero inventory with optional filtering via a
    /// custom where clause. Returns all tracked and untracked items.
    #[command(after_long_help = "\
EXAMPLES:
  xero items list
  xero items list --where 'IsSold==true'
  xero items list --where 'Code.StartsWith(\"WIDGET\")' --output json")]
    List {
        /// Custom Xero where clause filter expression
        ///
        /// Uses Xero's filter syntax: Field==Value, Field!=Value,
        /// Field.Contains("value"), Field.StartsWith("value").
        /// Multiple conditions joined with &&.
        /// Example: --where 'IsSold==true&&IsPurchased==true'
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },

    /// Get a specific item
    ///
    /// Retrieve full details for a single item by its UUID, including
    /// sales details, purchase details, and tracking information.
    #[command(after_long_help = "\
EXAMPLES:
  xero items get f3c7b4e2-1234-5678-abcd-ef9876543210
  xero items get f3c7b4e2-... --output json")]
    Get {
        /// Item ID (UUID)
        id: String,
    },

    /// Create an item
    ///
    /// Create a new inventory item with a unique code and name. Optionally
    /// set a description, sale/purchase unit prices, and the account codes
    /// for revenue and cost of goods sold.
    #[command(after_long_help = "\
EXAMPLES:
  xero items create --code WIDGET-01 --name \"Blue Widget\"
  xero items create --code SVC-100 --name \"Consulting Hour\" --sale-price 150.00 --sale-account 200
  xero items create --code MAT-50 --name \"Raw Steel\" --purchase-price 45.00 --purchase-account 630
  xero items create --code PART-A --name \"Part A\" --description \"Replacement part\" --sale-price 25.00 --sale-account 200 --purchase-price 10.00 --purchase-account 630")]
    Create {
        /// Unique item code (e.g. WIDGET-01, SKU-1234)
        #[arg(long)]
        code: String,
        /// Display name for the item
        #[arg(long)]
        name: String,
        /// Free-text description of the item
        #[arg(long)]
        description: Option<String>,
        /// Sale unit price (decimal, e.g. 25.00)
        #[arg(long)]
        sale_price: Option<String>,
        /// Revenue account code for sales (e.g. 200, 400)
        #[arg(long)]
        sale_account: Option<String>,
        /// Purchase unit price (decimal, e.g. 10.00)
        #[arg(long)]
        purchase_price: Option<String>,
        /// Cost-of-goods-sold account code for purchases (e.g. 630, 300)
        #[arg(long)]
        purchase_account: Option<String>,
    },

    /// Update an item
    ///
    /// Update an existing item's name, sale price, or purchase price.
    /// Only the fields you provide will be changed; all other fields
    /// remain untouched.
    #[command(after_long_help = "\
EXAMPLES:
  xero items update f3c7b4e2-... --name \"Red Widget\"
  xero items update f3c7b4e2-... --sale-price 175.00
  xero items update f3c7b4e2-... --name \"Updated Widget\" --sale-price 30.00 --purchase-price 12.50")]
    Update {
        /// Item ID (UUID) to update
        id: String,
        /// New display name for the item
        #[arg(long)]
        name: Option<String>,
        /// New sale unit price (decimal, e.g. 30.00)
        #[arg(long)]
        sale_price: Option<String>,
        /// New purchase unit price (decimal, e.g. 12.50)
        #[arg(long)]
        purchase_price: Option<String>,
    },

    /// Delete an item
    ///
    /// Permanently delete an item from the Xero inventory.
    /// The item must not be used on any transactions.
    #[command(after_long_help = "\
EXAMPLES:
  xero items delete f3c7b4e2-1234-5678-abcd-ef9876543210")]
    Delete {
        /// Item ID (UUID) to delete
        id: String,
    },

    /// View item history
    ///
    /// Retrieve the audit history for an item, showing all modifications
    /// and user actions.
    #[command(after_long_help = "\
EXAMPLES:
  xero items history f3c7b4e2-1234-5678-abcd-ef9876543210
  xero items history f3c7b4e2-... --output json")]
    History {
        /// Item ID (UUID)
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
