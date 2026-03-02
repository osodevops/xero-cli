use crate::api::endpoints::receipts;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::receipt::Receipt;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ReceiptCommands {
    /// List receipts
    ///
    /// Retrieve all receipts from Xero. Receipts are records of payments
    /// that have been received, typically for reimbursable expenses.
    /// Results include receipt number, contact, status, total, and date.
    #[command(after_long_help = "\
EXAMPLES:
  xero receipts list
  xero receipts list --output json
  xero receipts list --compact")]
    List,

    /// Get a specific receipt
    ///
    /// Fetch a single receipt by its Xero receipt ID. Returns full details
    /// including line items, contact information, and current status.
    #[command(after_long_help = "\
EXAMPLES:
  xero receipts get a1b2c3d4-e5f6-7890-abcd-ef1234567890
  xero receipts get a1b2c3d4-e5f6-7890-abcd-ef1234567890 --output json")]
    Get {
        /// The Xero receipt ID (UUID)
        id: String,
    },

    /// Create a receipt
    ///
    /// Create a new receipt in Xero from a JSON file. The file must contain
    /// a valid receipt payload including contact, line items, and user details.
    #[command(after_long_help = "\
EXAMPLES:
  xero receipts create --file receipt.json
  xero receipts create --file ./data/new-receipt.json --output json")]
    Create {
        /// Path to a JSON file containing the receipt payload
        #[arg(long)]
        file: String,
    },

    /// View receipt history
    ///
    /// Retrieve the change history for a specific receipt. Shows a timeline
    /// of modifications including status changes, edits, and user actions.
    #[command(after_long_help = "\
EXAMPLES:
  xero receipts history a1b2c3d4-e5f6-7890-abcd-ef1234567890
  xero receipts history a1b2c3d4-e5f6-7890-abcd-ef1234567890 --output json")]
    History {
        /// The Xero receipt ID (UUID)
        id: String,
    },
}

impl Tabular for Receipt {
    fn headers() -> Vec<String> {
        vec![
            "Number".into(),
            "Contact".into(),
            "Status".into(),
            "Total".into(),
            "Date".into(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.receipt_number.clone().unwrap_or_default(),
            self.contact
                .as_ref()
                .and_then(|c| c.name.clone())
                .unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.total.map(|t| t.to_string()).unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: ReceiptCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        ReceiptCommands::List => {
            let list = receipts::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ReceiptCommands::Get { id } => {
            let r = receipts::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&r, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ReceiptCommands::Create { file } => {
            let c = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let r = receipts::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&r, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ReceiptCommands::History { id } => {
            let records = receipts::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
