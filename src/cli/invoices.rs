use crate::api::endpoints::invoices::{self, InvoiceFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::invoice::Invoice;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum InvoiceCommands {
    /// List invoices
    List {
        /// Filter by status (DRAFT, SUBMITTED, AUTHORISED, PAID, VOIDED)
        #[arg(long)]
        status: Option<String>,
        /// Filter by contact name or ID
        #[arg(long)]
        contact: Option<String>,
        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// Filter to date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
        /// Order by (e.g. "DueDate DESC")
        #[arg(long)]
        order: Option<String>,
    },
    /// Get a specific invoice
    Get {
        /// Invoice ID
        id: String,
    },
    /// Create an invoice
    Create {
        /// JSON file with invoice data
        #[arg(long)]
        file: Option<String>,
        /// Contact name for inline creation
        #[arg(long)]
        contact: Option<String>,
        /// Line items: "Description,Quantity,UnitAmount"
        #[arg(long)]
        line_item: Vec<String>,
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_date: Option<String>,
    },
    /// Update an invoice
    Update {
        /// Invoice ID
        id: String,
        /// New status
        #[arg(long)]
        status: Option<String>,
        /// JSON file with update data
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
