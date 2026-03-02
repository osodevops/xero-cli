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
    ///
    /// Retrieves all credit notes in your Xero organisation, optionally
    /// filtered by status or a custom where clause. Credit notes represent
    /// amounts owed back to a contact and can be allocated against invoices.
    /// Results are returned in reverse chronological order.
    #[command(after_long_help = "\
EXAMPLES:
  xero credit-notes list
  xero credit-notes list --status AUTHORISED
  xero credit-notes list --status DRAFT --output json
  xero credit-notes list --where 'Total > 500.00'
  xero credit-notes list --status PAID --compact")]
    List {
        /// Filter by credit note status.
        /// Valid values: DRAFT, SUBMITTED, AUTHORISED, PAID, VOIDED
        #[arg(long)]
        status: Option<String>,
        /// Xero-style where clause for advanced filtering (e.g. 'Total > 500.00')
        #[arg(long, name = "where")]
        where_clause: Option<String>,
    },

    /// Get a specific credit note
    ///
    /// Retrieves the full details of a single credit note by its unique
    /// Xero identifier, including line items, contact information,
    /// remaining credit, and allocation history.
    #[command(after_long_help = "\
EXAMPLES:
  xero credit-notes get 249ae0af-bb1c-4b40-a88b-21e684740927
  xero credit-notes get 249ae0af-bb1c-4b40-a88b-21e684740927 --output json")]
    Get {
        /// Credit note UUID (e.g. 249ae0af-bb1c-4b40-a88b-21e684740927)
        id: String,
    },

    /// Create a credit note
    ///
    /// Creates a new credit note from a JSON file. The file must contain
    /// valid Xero CreditNote JSON including at minimum a Contact and at
    /// least one LineItem. The credit note will be created in DRAFT status
    /// unless a Status field is specified in the JSON payload.
    #[command(after_long_help = "\
EXAMPLES:
  xero credit-notes create --file credit-note.json
  xero credit-notes create --file credit-note.json --output json")]
    Create {
        /// Path to a JSON file containing the credit note payload
        #[arg(long)]
        file: String,
    },

    /// Allocate a credit note to an invoice
    ///
    /// Applies an amount from an existing credit note against an outstanding
    /// invoice. The credit note must be in AUTHORISED status and must have
    /// sufficient remaining credit. The allocation amount cannot exceed the
    /// remaining credit on the credit note or the amount due on the invoice.
    #[command(after_long_help = "\
EXAMPLES:
  xero credit-notes allocate 249ae0af-bb1c-4b40-a88b-21e684740927 \\
    --invoice 6b02e689-77c3-4515-a709-7e8b42889b69 \\
    --amount 150.00

  xero credit-notes allocate <CREDIT_NOTE_ID> \\
    --invoice <INVOICE_ID> \\
    --amount 500.00 --output json")]
    Allocate {
        /// Credit note UUID to allocate from
        id: String,
        /// Invoice UUID to apply the credit against
        #[arg(long)]
        invoice: String,
        /// Amount to allocate as a positive decimal (e.g. 150.00)
        #[arg(long)]
        amount: String,
    },

    /// View credit note history
    ///
    /// Retrieves the full audit history for a credit note, showing all
    /// status changes, edits, and allocation events in chronological order.
    #[command(after_long_help = "\
EXAMPLES:
  xero credit-notes history 249ae0af-bb1c-4b40-a88b-21e684740927
  xero credit-notes history 249ae0af-bb1c-4b40-a88b-21e684740927 --output json")]
    History {
        /// Credit note UUID to retrieve history for
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
