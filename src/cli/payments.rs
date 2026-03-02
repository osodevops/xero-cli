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
    List {
        /// Filter by invoice ID
        #[arg(long)]
        invoice: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },
    /// Get a specific payment
    Get {
        /// Payment ID
        id: String,
    },
    /// Create a payment
    Create {
        /// Invoice ID to pay
        #[arg(long)]
        invoice: String,
        /// Account ID to pay from
        #[arg(long)]
        account: String,
        /// Payment amount
        #[arg(long)]
        amount: String,
        /// Payment date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// Payment reference
        #[arg(long)]
        reference: Option<String>,
    },
    /// Delete (reverse) a payment
    Delete {
        /// Payment ID
        id: String,
    },
    /// View payment history
    History {
        /// Payment ID
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
