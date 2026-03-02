use crate::api::endpoints::budgets;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::budget::Budget;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BudgetCommands {
    /// List budgets
    List,
    /// Get a specific budget
    Get { id: String },
}

impl Tabular for Budget {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Type".to_string(),
            "Description".to_string(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.budget_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.budget_type.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: BudgetCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        BudgetCommands::List => {
            let list = budgets::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        BudgetCommands::Get { id } => {
            let budget = budgets::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&budget, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
