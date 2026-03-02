use crate::api::endpoints::payments::{self, PaymentFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::history::HistoryRecord;
use crate::models::payment::Payment;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum PaymentCommands {
    /// List payments
    ///
    /// Retrieve payments with optional filtering by invoice ID or a custom
    /// where clause. Returns all matching payments from the Xero organisation.
    #[command(after_long_help = "\
EXAMPLES:
  xero payments list
  xero payments list --invoice 243216c5-369e-4b40-b8d7-c9a3d50c2a12
  xero payments list --where 'Status==\"AUTHORISED\"'
  xero payments list --where 'Amount>1000.00' --output json")]
    List {
        /// Filter by invoice ID (UUID) to show only payments against that invoice
        #[arg(long)]
        invoice: Option<String>,
        /// Custom Xero where clause filter expression
        ///
        /// Uses Xero's filter syntax: Field==Value, Field!=Value,
        /// Field.Contains("value"), Field.StartsWith("value").
        /// Multiple conditions joined with &&.
        /// Example: --where 'Status=="AUTHORISED"&&Amount>500'
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },

    /// Get a specific payment
    ///
    /// Retrieve full details for a single payment by its UUID.
    #[command(after_long_help = "\
EXAMPLES:
  xero payments get a1b2c3d4-e5f6-7890-abcd-ef1234567890
  xero payments get a1b2c3d4-e5f6-7890-abcd-ef1234567890 --output json")]
    Get {
        /// Payment ID (UUID)
        id: String,
    },

    /// Create a payment
    ///
    /// Apply a payment to an existing invoice. Requires the invoice ID,
    /// the bank account ID to pay from, and the payment amount. Optionally
    /// set a payment date and reference string.
    #[command(after_long_help = "\
EXAMPLES:
  xero payments create --invoice 243216c5-... --account 7d05a53d-... --amount 500.00
  xero payments create --invoice 243216c5-... --account 7d05a53d-... --amount 250.00 --date 2024-06-15
  xero payments create --invoice 243216c5-... --account 7d05a53d-... --amount 100.00 --reference \"June payment\"")]
    Create {
        /// Invoice ID (UUID) to apply the payment against
        #[arg(long)]
        invoice: String,
        /// Bank account ID (UUID) to pay from
        #[arg(long)]
        account: String,
        /// Payment amount (decimal, e.g. 150.00)
        #[arg(long)]
        amount: String,
        /// Payment date in YYYY-MM-DD format (defaults to today if omitted)
        #[arg(long)]
        date: Option<String>,
        /// Free-text reference that appears on the payment
        #[arg(long)]
        reference: Option<String>,
    },

    /// Delete (reverse) a payment
    ///
    /// Reverse an existing payment by setting its status to DELETED.
    /// This does not remove the record; the payment remains visible with
    /// a DELETED status.
    #[command(after_long_help = "\
EXAMPLES:
  xero payments delete a1b2c3d4-e5f6-7890-abcd-ef1234567890")]
    Delete {
        /// Payment ID (UUID) to reverse
        id: String,
    },

    /// View payment history
    ///
    /// Retrieve the audit history for a payment, showing all status
    /// changes, modifications, and user actions.
    #[command(after_long_help = "\
EXAMPLES:
  xero payments history a1b2c3d4-e5f6-7890-abcd-ef1234567890
  xero payments history a1b2c3d4-... --output json")]
    History {
        /// Payment ID (UUID)
        id: String,
    },
}

impl Tabular for Payment {
    fn headers() -> Vec<String> {
        vec![
            "Payment ID".to_string(),
            "Invoice".to_string(),
            "Account".to_string(),
            "Amount".to_string(),
            "Date".to_string(),
            "Status".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.payment_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.invoice
                .as_ref()
                .and_then(|i| i.invoice_number.clone())
                .unwrap_or_default(),
            self.account
                .as_ref()
                .and_then(|a| a.code.clone())
                .unwrap_or_default(),
            self.amount.map(|a| a.to_string()).unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
            self.status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
        ]
    }
}

impl Tabular for HistoryRecord {
    fn headers() -> Vec<String> {
        vec![
            "Date".to_string(),
            "Changes".to_string(),
            "User".to_string(),
            "Details".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.date_utc.clone().unwrap_or_default(),
            self.changes.clone().unwrap_or_default(),
            self.user.clone().unwrap_or_default(),
            self.details.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: PaymentCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        PaymentCommands::List {
            invoice,
            where_clause,
        } => {
            let filters = PaymentFilters {
                invoice_id: invoice,
                where_clause,
                ..Default::default()
            };
            let payments_list = payments::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&payments_list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PaymentCommands::Get { id } => {
            let payment = payments::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&payment, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PaymentCommands::Create {
            invoice,
            account,
            amount,
            date,
            reference,
        } => {
            let mut body = serde_json::json!({
                "Invoice": {"InvoiceID": invoice},
                "Account": {"AccountID": account},
                "Amount": amount,
            });
            if let Some(d) = date {
                body["Date"] = serde_json::Value::String(d);
            }
            if let Some(r) = reference {
                body["Reference"] = serde_json::Value::String(r);
            }

            let payment = payments::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&payment, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PaymentCommands::Delete { id } => {
            let payment = payments::delete(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Payment deleted successfully.");
            let rendered = output::render_single(&payment, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        PaymentCommands::History { id } => {
            let records = payments::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
