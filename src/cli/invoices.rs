use crate::api::endpoints::invoices::{self, InvoiceFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::invoice::Invoice;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum InvoiceCommands {
    /// List invoices
    ///
    /// Retrieve invoices with optional filtering by status, contact, date range,
    /// and custom where clauses. Results are paginated — use --all-pages to fetch all.
    #[command(after_long_help = "\
EXAMPLES:
  xero invoices list
  xero invoices list --status AUTHORISED
  xero invoices list --status PAID --contact \"Acme Corp\"
  xero invoices list --from 2024-01-01 --to 2024-06-30
  xero invoices list --where \"Type==\\\"ACCREC\\\"\" --order \"DueDate DESC\"
  xero invoices list --output json --all-pages")]
    List {
        /// Filter by status: DRAFT, SUBMITTED, AUTHORISED, PAID, VOIDED, DELETED
        #[arg(long)]
        status: Option<String>,
        /// Filter by contact name or contact ID (UUID)
        #[arg(long)]
        contact: Option<String>,
        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// Filter to date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
        /// Custom Xero where clause filter expression
        ///
        /// Uses Xero's filter syntax: Field==Value, Field!=Value,
        /// Field.Contains("value"), Field.StartsWith("value").
        /// Multiple conditions joined with &&.
        /// Example: --where 'Type=="ACCREC"&&Status=="AUTHORISED"'
        #[arg(long, name = "where")]
        where_clause: Option<String>,
        /// Order by field and direction (e.g. "DueDate DESC", "InvoiceNumber ASC")
        #[arg(long)]
        order: Option<String>,
    },

    /// Get a specific invoice
    ///
    /// Retrieve full details for a single invoice by its UUID.
    #[command(after_long_help = "\
EXAMPLES:
  xero invoices get 243216c5-369e-4b40-b8d7-c9a3d50c2a12
  xero invoices get 243216c5-369e-4b40-b8d7-c9a3d50c2a12 --output json")]
    Get {
        /// Invoice ID (UUID)
        id: String,
    },

    /// Create an invoice
    ///
    /// Create a new invoice either from a JSON file or inline with flags.
    /// Inline creation builds an ACCREC (accounts receivable / sales) invoice.
    /// For complex invoices or ACCPAY (bills), use --file with a JSON payload.
    #[command(after_long_help = "\
EXAMPLES:
  xero invoices create --contact \"Acme Corp\" --line-item \"Consulting,10,150.00\"
  xero invoices create --contact \"Acme Corp\" --line-item \"Design,5,200.00\" --due-date 2024-12-31
  xero invoices create --file invoice.json")]
    Create {
        /// Path to JSON file with full invoice data
        #[arg(long)]
        file: Option<String>,
        /// Contact name for inline creation (required unless using --file)
        #[arg(long)]
        contact: Option<String>,
        /// Line items in format "Description,Quantity,UnitAmount" (repeatable)
        #[arg(long)]
        line_item: Vec<String>,
        /// Due date in YYYY-MM-DD format
        #[arg(long)]
        due_date: Option<String>,
    },

    /// Update an invoice
    ///
    /// Update an existing invoice by changing its status or providing a JSON payload.
    /// Common status transitions: DRAFT -> SUBMITTED -> AUTHORISED.
    /// Use VOIDED to void an authorised invoice.
    #[command(after_long_help = "\
EXAMPLES:
  xero invoices update 243216c5-... --status AUTHORISED
  xero invoices update 243216c5-... --status VOIDED
  xero invoices update 243216c5-... --file updates.json")]
    Update {
        /// Invoice ID (UUID)
        id: String,
        /// New status: DRAFT, SUBMITTED, AUTHORISED, VOIDED
        #[arg(long)]
        status: Option<String>,
        /// Path to JSON file with update data
        #[arg(long)]
        file: Option<String>,
    },
}

impl Tabular for Invoice {
    fn headers() -> Vec<String> {
        vec![
            "Invoice #".to_string(),
            "Type".to_string(),
            "Contact".to_string(),
            "Status".to_string(),
            "Amount".to_string(),
            "Due".to_string(),
            "Due Date".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.invoice_number.clone().unwrap_or_default(),
            self.invoice_type
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
            self.amount_due.map(|a| a.to_string()).unwrap_or_default(),
            self.due_date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: InvoiceCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        InvoiceCommands::List {
            status,
            contact,
            from: _,
            to: _,
            where_clause,
            order,
        } => {
            let filters = InvoiceFilters {
                status,
                contact_id: contact,
                where_clause,
                order,
                page: Some(1),
                page_size: Some(global.page_size),
                ..Default::default()
            };
            let invoices_list = invoices::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&invoices_list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        InvoiceCommands::Get { id } => {
            let invoice = invoices::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&invoice, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        InvoiceCommands::Create {
            file,
            contact,
            line_item,
            due_date,
        } => {
            let body = if let Some(file_path) = file {
                let content = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else {
                build_inline_invoice(contact, &line_item, due_date)?
            };

            let invoice = invoices::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&invoice, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        InvoiceCommands::Update { id, status, file } => {
            let body = if let Some(file_path) = file {
                let content = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else if let Some(status) = status {
                serde_json::json!({"Status": status})
            } else {
                return Err(miette::miette!(
                    "Provide --file or --status to update an invoice"
                ));
            };

            let invoice = invoices::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&invoice, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}

fn build_inline_invoice(
    contact: Option<String>,
    line_items: &[String],
    due_date: Option<String>,
) -> miette::Result<serde_json::Value> {
    let contact_name = contact
        .ok_or_else(|| miette::miette!("--contact is required for inline invoice creation"))?;

    let items: Vec<serde_json::Value> = line_items
        .iter()
        .map(|li| {
            let parts: Vec<&str> = li.split(',').collect();
            if parts.len() != 3 {
                return Err(miette::miette!(
                    "Line item format: \"Description,Quantity,UnitAmount\". Got: {li}"
                ));
            }
            Ok(serde_json::json!({
                "Description": parts[0].trim(),
                "Quantity": parts[1].trim(),
                "UnitAmount": parts[2].trim(),
            }))
        })
        .collect::<miette::Result<_>>()?;

    let mut invoice = serde_json::json!({
        "Type": "ACCREC",
        "Contact": {"Name": contact_name},
        "LineItems": items,
    });

    if let Some(due) = due_date {
        invoice["DueDate"] = serde_json::Value::String(due);
    }

    Ok(invoice)
}
