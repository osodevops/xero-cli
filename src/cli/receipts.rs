use crate::api::endpoints::receipts;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::receipt::Receipt;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ReceiptCommands {
    /// List receipts
    List,
    /// Get a specific receipt
    Get { id: String },
    /// Create a receipt
    Create {
        #[arg(long)]
        file: String,
    },
    /// View receipt history
    History { id: String },
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
