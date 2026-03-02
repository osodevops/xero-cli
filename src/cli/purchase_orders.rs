use crate::api::endpoints::purchase_orders::{self, PurchaseOrderFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::purchase_order::PurchaseOrder;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum PurchaseOrderCommands {
    /// List purchase orders
    List {
        /// Filter by status (DRAFT, SUBMITTED, AUTHORISED, BILLED, DELETED)
        #[arg(long)]
        status: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },
    /// Get a specific purchase order
    Get {
        /// Purchase order ID
        id: String,
    },
    /// Create a purchase order
    Create {
        /// Contact name
        #[arg(long)]
        contact: Option<String>,
        /// JSON file with purchase order data
        #[arg(long)]
        file: Option<String>,
    },
    /// View purchase order history
    History {
        /// Purchase order ID
        id: String,
    },
}

impl Tabular for PurchaseOrder {
    fn headers() -> Vec<String> {
        vec![
            "PO #".to_string(),
            "Contact".to_string(),
            "Status".to_string(),
            "Total".to_string(),
            "Date".to_string(),
            "Delivery".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.purchase_order_number.clone().unwrap_or_default(),
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
            self.delivery_date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: PurchaseOrderCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        PurchaseOrderCommands::List {
            status,
            where_clause,
        } => {
            let filters = PurchaseOrderFilters {
                status,
                where_clause,
                ..Default::default()
            };
            let list = purchase_orders::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PurchaseOrderCommands::Get { id } => {
            let po = purchase_orders::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&po, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PurchaseOrderCommands::Create { contact, file } => {
            let body = if let Some(file_path) = file {
                let content = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else if let Some(contact_name) = contact {
                serde_json::json!({"Contact": {"Name": contact_name}})
            } else {
                return Err(miette::miette!(
                    "Provide --file or --contact for purchase order creation"
                ));
            };

            let po = purchase_orders::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&po, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PurchaseOrderCommands::History { id } => {
            let records = purchase_orders::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
