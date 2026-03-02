use crate::api::endpoints::prepayments;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::prepayment::Prepayment;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum PrepaymentCommands {
    /// List prepayments
    List,
    /// Get a specific prepayment
    Get {
        /// Prepayment ID
        id: String,
    },
    /// Allocate a prepayment to an invoice
    Allocate {
        /// Prepayment ID
        id: String,
        /// Invoice ID to allocate to
        #[arg(long)]
        invoice: String,
        /// Amount to allocate
        #[arg(long)]
        amount: String,
    },
    /// View prepayment history
    History {
        /// Prepayment ID
        id: String,
    },
}

impl Tabular for Prepayment {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Type".to_string(),
            "Contact".to_string(),
            "Status".to_string(),
            "Total".to_string(),
            "Remaining".to_string(),
            "Date".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.prepayment_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.prepayment_type.clone().unwrap_or_default(),
            self.contact
                .as_ref()
                .and_then(|c| c.name.clone())
                .unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.total.map(|t| t.to_string()).unwrap_or_default(),
            self.remaining_credit
                .map(|r| r.to_string())
                .unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: PrepaymentCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        PrepaymentCommands::List => {
            let list = prepayments::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PrepaymentCommands::Get { id } => {
            let pp = prepayments::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&pp, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PrepaymentCommands::Allocate {
            id,
            invoice,
            amount,
        } => {
            let body = serde_json::json!({
                "Allocations": [{
                    "Invoice": {"InvoiceID": invoice},
                    "Amount": amount,
                }]
            });
            let allocations = prepayments::allocate(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&allocations, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PrepaymentCommands::History { id } => {
            let records = prepayments::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
