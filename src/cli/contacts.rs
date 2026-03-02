use crate::api::endpoints::contacts::{self, ContactFilters};
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::contact::Contact;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ContactCommands {
    /// List contacts
    List {
        /// Search by name
        #[arg(long)]
        search: Option<String>,
        /// Custom where clause
        #[arg(long, name = "where")]
        where_clause: Option<String>,
        /// Order by
        #[arg(long)]
        order: Option<String>,
    },
    /// Get a specific contact
    Get {
        /// Contact ID
        id: String,
    },
    /// Create a contact
    Create {
        /// Contact name
        #[arg(long)]
        name: String,
        /// Email address
        #[arg(long)]
        email: Option<String>,
        /// Tax number
        #[arg(long)]
        tax_number: Option<String>,
        /// JSON file with contact data
        #[arg(long)]
        file: Option<String>,
    },
    /// Update a contact
    Update {
        /// Contact ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New email
        #[arg(long)]
        email: Option<String>,
        /// JSON file with update data
        #[arg(long)]
        file: Option<String>,
    },
}

impl Tabular for Contact {
    fn headers() -> Vec<String> {
        vec![
            "Contact ID".to_string(),
            "Name".to_string(),
            "Email".to_string(),
            "Status".to_string(),
            "Customer".to_string(),
            "Supplier".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.contact_id
                .as_deref()
                .unwrap_or("")
                .chars()
                .take(8)
                .collect::<String>()
                + "...",
            self.name.clone().unwrap_or_default(),
            self.email_address.clone().unwrap_or_default(),
            self.contact_status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            self.is_customer
                .map(|b| if b { "Yes" } else { "No" })
                .unwrap_or("")
                .to_string(),
            self.is_supplier
                .map(|b| if b { "Yes" } else { "No" })
                .unwrap_or("")
                .to_string(),
        ]
    }
}

pub async fn execute(command: ContactCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        ContactCommands::List {
            search,
            where_clause,
            order,
        } => {
            let filters = ContactFilters {
                search,
                where_clause,
                order,
                page: Some(1),
                page_size: Some(global.page_size),
                ..Default::default()
            };
            let contacts_list = contacts::list(&client, &filters)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&contacts_list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        ContactCommands::Get { id } => {
            let contact = contacts::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&contact, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        ContactCommands::Create {
            name,
            email,
            tax_number,
            file,
        } => {
            let body = if let Some(file_path) = file {
                let content = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else {
                let mut contact = serde_json::json!({"Name": name});
                if let Some(email) = email {
                    contact["EmailAddress"] = serde_json::Value::String(email);
                }
                if let Some(tax) = tax_number {
                    contact["TaxNumber"] = serde_json::Value::String(tax);
                }
                contact
            };

            let contact = contacts::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&contact, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        ContactCommands::Update {
            id,
            name,
            email,
            file,
        } => {
            let body = if let Some(file_path) = file {
                let content = std::fs::read_to_string(&file_path)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else {
                let mut updates = serde_json::json!({});
                if let Some(name) = name {
                    updates["Name"] = serde_json::Value::String(name);
                }
                if let Some(email) = email {
                    updates["EmailAddress"] = serde_json::Value::String(email);
                }
                updates
            };

            let contact = contacts::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&contact, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
