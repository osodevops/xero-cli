use crate::api::endpoints::credit_notes::{self, CreditNoteFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::allocation::Allocation;
use crate::models::credit_note::CreditNote;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum CreditNoteCommands {
    /// List credit notes
    List {
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },
    /// Get a specific credit note
    Get {
        /// Credit note ID
        id: String,
    },
    /// Create a credit note
    Create {
        /// JSON file with credit note data
        #[arg(long)]
        file: String,
    },
    /// Allocate a credit note to an invoice
    Allocate {
        /// Credit note ID
        id: String,
        /// Invoice ID to allocate to
        #[arg(long)]
        invoice: String,
        /// Amount to allocate
        #[arg(long)]
        amount: String,
    },
    /// View credit note history
    History {
        /// Credit note ID
        id: String,
    },
}

impl Tabular for CreditNote {
    fn headers() -> Vec<String> {
        vec![
            "Number".to_string(),
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
            self.credit_note_number.clone().unwrap_or_default(),
            self.credit_note_type
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
            self.contact
                .as_ref()
                .and_then(|c| c.name.clone())
                .unwrap_or_default(),
            self.status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            self.total.map(|t| t.to_string()).unwrap_or_default(),
            self.remaining_credit
                .map(|r| r.to_string())
                .unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
        ]
    }
}

impl Tabular for Allocation {
    fn headers() -> Vec<String> {
        vec![
            "Allocation ID".to_string(),
            "Invoice".to_string(),
            "Amount".to_string(),
            "Date".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.allocation_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.invoice
                .as_ref()
                .and_then(|i| i.invoice_number.clone())
                .unwrap_or_default(),
            self.amount.map(|a| a.to_string()).unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: CreditNoteCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        CreditNoteCommands::List {
            status,
            where_clause,
        } => {
            let filters = CreditNoteFilters {
                status,
                where_clause,
                ..Default::default()
            };
            let list = credit_notes::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        CreditNoteCommands::Get { id } => {
            let cn = credit_notes::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&cn, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        CreditNoteCommands::Create { file } => {
            let content = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let cn = credit_notes::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&cn, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        CreditNoteCommands::Allocate {
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
            let allocations = credit_notes::allocate(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&allocations, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        CreditNoteCommands::History { id } => {
            let records = credit_notes::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
