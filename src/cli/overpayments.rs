use crate::api::endpoints::overpayments;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::overpayment::Overpayment;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum OverpaymentCommands {
    /// List overpayments
    ///
    /// Retrieve all overpayments for the connected Xero organisation.
    /// Overpayments arise when a contact is paid more than is owed.
    #[command(after_long_help = "\
EXAMPLES:
  xero overpayments list
  xero overpayments list --output json
  xero overpayments list --compact")]
    List,

    /// Get a specific overpayment
    ///
    /// Retrieve full details for a single overpayment by its UUID,
    /// including allocations, line items, and remaining credit.
    #[command(after_long_help = "\
EXAMPLES:
  xero overpayments get 8c0e4afe-3b0d-4a38-85fc-a1b20928a6e3
  xero overpayments get 8c0e4afe-3b0d-4a38-85fc-a1b20928a6e3 --output json")]
    Get {
        /// Overpayment ID (UUID)
        id: String,
    },

    /// Allocate an overpayment to an invoice
    ///
    /// Apply part or all of an overpayment's remaining credit against an
    /// outstanding invoice. The amount must not exceed the remaining credit
    /// on the overpayment or the amount due on the invoice.
    #[command(after_long_help = "\
EXAMPLES:
  xero overpayments allocate 8c0e4afe-... --invoice 243216c5-... --amount 150.00
  xero overpayments allocate 8c0e4afe-... --invoice 243216c5-... --amount 50.00 --output json")]
    Allocate {
        /// Overpayment ID (UUID) to allocate from
        id: String,
        /// Invoice ID (UUID) to allocate the overpayment against
        #[arg(long)]
        invoice: String,
        /// Amount to allocate (must not exceed remaining credit or invoice balance)
        #[arg(long)]
        amount: String,
    },

    /// View overpayment history
    ///
    /// Retrieve the audit history for an overpayment, showing all changes
    /// and events recorded against it.
    #[command(after_long_help = "\
EXAMPLES:
  xero overpayments history 8c0e4afe-3b0d-4a38-85fc-a1b20928a6e3
  xero overpayments history 8c0e4afe-... --output json")]
    History {
        /// Overpayment ID (UUID)
        id: String,
    },
}

impl Tabular for Overpayment {
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
            self.overpayment_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.overpayment_type.clone().unwrap_or_default(),
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

pub async fn execute(command: OverpaymentCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        OverpaymentCommands::List => {
            let list = overpayments::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        OverpaymentCommands::Get { id } => {
            let op = overpayments::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&op, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        OverpaymentCommands::Allocate {
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
            let allocations = overpayments::allocate(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&allocations, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        OverpaymentCommands::History { id } => {
            let records = overpayments::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
