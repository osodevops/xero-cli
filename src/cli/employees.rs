use crate::api::endpoints::employees;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::employee::Employee;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum EmployeeCommands {
    /// List employees
    List,
    /// Get a specific employee
    Get { id: String },
}

impl Tabular for Employee {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "First Name".to_string(),
            "Last Name".to_string(),
            "Status".to_string(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.employee_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.first_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: EmployeeCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        EmployeeCommands::List => {
            let list = employees::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        EmployeeCommands::Get { id } => {
            let emp = employees::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&emp, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
