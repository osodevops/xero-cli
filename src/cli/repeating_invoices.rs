use crate::api::endpoints::repeating_invoices;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::repeating_invoice::RepeatingInvoice;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum RepeatingInvoiceCommands {
    /// List repeating invoices
    ///
    /// Retrieve all repeating invoice templates in the connected Xero organisation.
    /// Shows the invoice type, contact, status, and total for each template.
    #[command(after_long_help = "\
EXAMPLES:
  xero repeating-invoices list
  xero repeating-invoices list --output json
  xero repeating-invoices list --compact")]
    List,

    /// Get a specific repeating invoice
    ///
    /// Retrieve full details for a single repeating invoice template by its UUID,
    /// including schedule, line items, and contact information.
    #[command(after_long_help = "\
EXAMPLES:
  xero repeating-invoices get 8b2e4f6a-3c1d-5e7f-9a0b-c2d4e6f8a1b3
  xero repeating-invoices get 8b2e4f6a-3c1d-5e7f-9a0b-c2d4e6f8a1b3 --output json")]
    Get {
        /// Repeating invoice ID (UUID)
        id: String,
    },
}

impl Tabular for RepeatingInvoice {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Type".to_string(),
            "Contact".to_string(),
            "Status".to_string(),
            "Total".to_string(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.repeating_invoice_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.invoice_type.clone().unwrap_or_default(),
            self.contact
                .as_ref()
                .and_then(|c| c.name.clone())
                .unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.total.map(|t| t.to_string()).unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: RepeatingInvoiceCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        RepeatingInvoiceCommands::List => {
            let list = repeating_invoices::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        RepeatingInvoiceCommands::Get { id } => {
            let ri = repeating_invoices::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&ri, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
