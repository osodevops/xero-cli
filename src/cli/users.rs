use crate::api::endpoints::users;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::user::User;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum UserCommands {
    /// List users
    ///
    /// Retrieve all users who have access to the connected Xero organisation.
    /// Each user includes their email address, name, and organisation role.
    #[command(after_long_help = "\
EXAMPLES:
  xero users list
  xero users list --output json
  xero users list --compact")]
    List,

    /// Get a specific user
    ///
    /// Retrieve full details for a single user by their UUID,
    /// including email, name, and assigned organisation role.
    #[command(after_long_help = "\
EXAMPLES:
  xero users get f7c8d9e0-1234-5678-9abc-def012345678
  xero users get f7c8d9e0-1234-5678-9abc-def012345678 --output json")]
    Get {
        /// User ID (UUID)
        id: String,
    },
}

impl Tabular for User {
    fn headers() -> Vec<String> {
        vec![
            "Email".to_string(),
            "First Name".to_string(),
            "Last Name".to_string(),
            "Role".to_string(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.email_address.clone().unwrap_or_default(),
            self.first_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
            self.organisation_role.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: UserCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        UserCommands::List => {
            let list = users::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        UserCommands::Get { id } => {
            let user = users::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&user, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
