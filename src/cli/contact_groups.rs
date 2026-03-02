use crate::api::endpoints::contact_groups;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::contact_group::ContactGroup;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ContactGroupCommands {
    /// List contact groups
    List,
    /// Get a specific contact group
    Get { id: String },
    /// Create a contact group
    Create {
        #[arg(long)]
        name: String,
    },
    /// Update a contact group
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete a contact group
    Delete { id: String },
}

impl Tabular for ContactGroup {
    fn headers() -> Vec<String> {
        vec![
            "ID".into(),
            "Name".into(),
            "Status".into(),
            "Contacts".into(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.contact_group_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.name.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.contacts.len().to_string(),
        ]
    }
}

pub async fn execute(command: ContactGroupCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        ContactGroupCommands::List => {
            let list = contact_groups::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ContactGroupCommands::Get { id } => {
            let cg = contact_groups::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&cg, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ContactGroupCommands::Create { name } => {
            let body = serde_json::json!({"Name": name});
            let cg = contact_groups::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&cg, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ContactGroupCommands::Update { id, name } => {
            let mut body = serde_json::json!({});
            if let Some(n) = name {
                body["Name"] = serde_json::Value::String(n);
            }
            let cg = contact_groups::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&cg, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ContactGroupCommands::Delete { id } => {
            contact_groups::delete(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            eprintln!("Contact group deleted successfully.");
        }
    }
    Ok(())
}
