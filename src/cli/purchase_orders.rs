use crate::api::endpoints::purchase_orders::{self, PurchaseOrderFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::purchase_order::PurchaseOrder;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum PurchaseOrderCommands {
    /// List purchase orders
    ///
    /// Retrieves all purchase orders in your Xero organisation, optionally
    /// filtered by status or a custom where clause. Purchase orders track
    /// goods or services ordered from suppliers. Results are returned in
    /// reverse chronological order.
    #[command(after_long_help = "\
EXAMPLES:
  xero purchase-orders list
  xero purchase-orders list --status AUTHORISED
  xero purchase-orders list --status DRAFT --output json
  xero purchase-orders list --where 'Total > 1000.00'
  xero purchase-orders list --status BILLED --compact")]
    List {
        /// Filter by purchase order status.
        /// Valid values: DRAFT, SUBMITTED, AUTHORISED, BILLED, DELETED
        #[arg(long)]
        status: Option<String>,
        /// Xero-style where clause for advanced filtering (e.g. 'Total > 1000.00')
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },

    /// Get a specific purchase order
    ///
    /// Retrieves the full details of a single purchase order by its unique
    /// Xero identifier, including line items, contact information, delivery
    /// date, and current status.
    #[command(after_long_help = "\
EXAMPLES:
  xero purchase-orders get 7e5a3d09-46f0-4fc0-8e06-97da2510cb8f
  xero purchase-orders get 7e5a3d09-46f0-4fc0-8e06-97da2510cb8f --output json")]
    Get {
        /// Purchase order UUID (e.g. 7e5a3d09-46f0-4fc0-8e06-97da2510cb8f)
        id: String,
    },

    /// Create a purchase order
    ///
    /// Creates a new purchase order, either from a JSON file containing the
    /// full payload or by specifying a contact name for a minimal order.
    /// When using --file, the JSON must include at minimum a Contact and at
    /// least one LineItem. When using --contact, a skeleton order is created
    /// for the named contact. You must provide either --file or --contact.
    #[command(after_long_help = "\
EXAMPLES:
  xero purchase-orders create --file po.json
  xero purchase-orders create --contact 'Acme Supplies'
  xero purchase-orders create --file po.json --output json")]
    Create {
        /// Contact name to create a minimal purchase order for
        #[arg(long)]
        contact: Option<String>,
        /// Path to a JSON file containing the full purchase order payload
        #[arg(long)]
        file: Option<String>,
    },

    /// View purchase order history
    ///
    /// Retrieves the full audit history for a purchase order, showing all
    /// status changes and edits in chronological order.
    #[command(after_long_help = "\
EXAMPLES:
  xero purchase-orders history 7e5a3d09-46f0-4fc0-8e06-97da2510cb8f
  xero purchase-orders history 7e5a3d09-46f0-4fc0-8e06-97da2510cb8f --output json")]
    History {
        /// Purchase order UUID to retrieve history for
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
