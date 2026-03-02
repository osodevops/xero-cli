use crate::api::endpoints::contact_groups;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::contact_group::ContactGroup;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ContactGroupCommands {
    /// List contact groups
    ///
    /// Retrieve all contact groups defined in the organisation.
    /// Contact groups allow you to organise contacts into categories
    /// for reporting or bulk operations.
    #[command(after_long_help = "\
EXAMPLES:
  xero contact-groups list
  xero contact-groups list --output json")]
    List,

    /// Get a specific contact group
    ///
    /// Retrieve full details for a single contact group by its UUID,
    /// including the contacts that belong to it.
    #[command(after_long_help = "\
EXAMPLES:
  xero contact-groups get a1b2c3d4-e5f6-7890-abcd-ef1234567890
  xero contact-groups get a1b2c3d4-e5f6-7890-abcd-ef1234567890 --output json")]
    Get {
        /// Contact group ID (UUID)
        id: String,
    },

    /// Create a contact group
    ///
    /// Create a new contact group with the given name.
    /// Group names must be unique within the organisation.
    #[command(after_long_help = "\
EXAMPLES:
  xero contact-groups create --name \"Key Clients\"
  xero contact-groups create --name \"Suppliers - APAC\"")]
    Create {
        /// Name for the new contact group (must be unique)
        #[arg(long)]
        name: String,
    },

    /// Update a contact group
    ///
    /// Update the name of an existing contact group.
    /// Only the name field can be modified via this command.
    #[command(after_long_help = "\
EXAMPLES:
  xero contact-groups update a1b2c3d4-... --name \"VIP Clients\"
  xero contact-groups update a1b2c3d4-... --name \"Suppliers - Europe\"")]
    Update {
        /// Contact group ID (UUID)
        id: String,
        /// New name for the contact group
        #[arg(long)]
        name: Option<String>,
    },

    /// Delete a contact group
    ///
    /// Permanently delete a contact group by its UUID.
    /// This removes the group but does not delete the contacts within it.
    #[command(after_long_help = "\
EXAMPLES:
  xero contact-groups delete a1b2c3d4-e5f6-7890-abcd-ef1234567890")]
    Delete {
        /// Contact group ID (UUID)
        id: String,
    },
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
