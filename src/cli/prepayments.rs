use crate::api::endpoints::prepayments;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::prepayment::Prepayment;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum PrepaymentCommands {
    /// List prepayments
    ///
    /// Retrieve all prepayments for the connected Xero organisation.
    /// Prepayments represent payments made to a contact before an invoice
    /// has been raised (e.g. deposits or advance payments).
    #[command(after_long_help = "\
EXAMPLES:
  xero prepayments list
  xero prepayments list --output json
  xero prepayments list --compact")]
    List,

    /// Get a specific prepayment
    ///
    /// Retrieve full details for a single prepayment by its UUID,
    /// including allocations, line items, and remaining credit.
    #[command(after_long_help = "\
EXAMPLES:
  xero prepayments get a1f02b7c-51d8-4e2a-b6cc-4ae9e7d5a123
  xero prepayments get a1f02b7c-51d8-4e2a-b6cc-4ae9e7d5a123 --output json")]
    Get {
        /// Prepayment ID (UUID)
        id: String,
    },

    /// Allocate a prepayment to an invoice
    ///
    /// Apply part or all of a prepayment's remaining credit against an
    /// outstanding invoice. The amount must not exceed the remaining credit
    /// on the prepayment or the amount due on the invoice.
    #[command(after_long_help = "\
EXAMPLES:
  xero prepayments allocate a1f02b7c-... --invoice 243216c5-... --amount 250.00
  xero prepayments allocate a1f02b7c-... --invoice 243216c5-... --amount 75.50 --output json")]
    Allocate {
        /// Prepayment ID (UUID) to allocate from
        id: String,
        /// Invoice ID (UUID) to allocate the prepayment against
        #[arg(long)]
        invoice: String,
        /// Amount to allocate (must not exceed remaining credit or invoice balance)
        #[arg(long)]
        amount: String,
    },

    /// View prepayment history
    ///
    /// Retrieve the audit history for a prepayment, showing all changes
    /// and events recorded against it.
    #[command(after_long_help = "\
EXAMPLES:
  xero prepayments history a1f02b7c-51d8-4e2a-b6cc-4ae9e7d5a123
  xero prepayments history a1f02b7c-... --output json")]
    History {
        /// Prepayment ID (UUID)
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
