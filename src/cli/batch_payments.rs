use crate::api::endpoints::batch_payments;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::batch_payment::BatchPayment;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BatchPaymentCommands {
    /// List batch payments
    List,
    /// Get a specific batch payment
    Get { id: String },
    /// Create a batch payment
    Create {
        #[arg(long)]
        file: String,
    },
    /// Delete a batch payment
    Delete { id: String },
}

impl Tabular for BatchPayment {
    fn headers() -> Vec<String> {
        vec![
            "ID".into(),
            "Account".into(),
            "Status".into(),
            "Total".into(),
            "Date".into(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.batch_payment_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.account
                .as_ref()
                .and_then(|a| a.code.clone())
                .unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.total_amount.map(|t| t.to_string()).unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: BatchPaymentCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        BatchPaymentCommands::List => {
            let list = batch_payments::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        BatchPaymentCommands::Get { id } => {
            let bp = batch_payments::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&bp, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        BatchPaymentCommands::Create { file } => {
            let c = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let bp = batch_payments::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&bp, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        BatchPaymentCommands::Delete { id } => {
            let bp = batch_payments::delete(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Batch payment deleted successfully.");
            let rendered = output::render_single(&bp, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
