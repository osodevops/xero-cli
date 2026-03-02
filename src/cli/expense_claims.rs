use crate::api::endpoints::expense_claims;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::expense_claim::ExpenseClaim;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ExpenseClaimCommands {
    /// List expense claims
    List,
    /// Get a specific expense claim
    Get { id: String },
    /// Create an expense claim
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update an expense claim
    Update {
        id: String,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        file: Option<String>,
    },
    /// View expense claim history
    History { id: String },
}

impl Tabular for ExpenseClaim {
    fn headers() -> Vec<String> {
        vec![
            "ID".into(),
            "User".into(),
            "Status".into(),
            "Total".into(),
            "Due".into(),
        ]
    }
    fn row(&self) -> Vec<String> {
        let user_name = self
            .user
            .as_ref()
            .map(|u| {
                format!(
                    "{} {}",
                    u.first_name.as_deref().unwrap_or(""),
                    u.last_name.as_deref().unwrap_or("")
                )
                .trim()
                .to_string()
            })
            .unwrap_or_default();
        vec![
            self.expense_claim_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            user_name,
            self.status.clone().unwrap_or_default(),
            self.total.map(|t| t.to_string()).unwrap_or_default(),
            self.amount_due.map(|a| a.to_string()).unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: ExpenseClaimCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        ExpenseClaimCommands::List => {
            let list = expense_claims::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ExpenseClaimCommands::Get { id } => {
            let ec = expense_claims::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&ec, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ExpenseClaimCommands::Create { file } => {
            let c = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let ec = expense_claims::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&ec, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ExpenseClaimCommands::Update { id, status, file } => {
            let body = if let Some(fp) = file {
                let c = std::fs::read_to_string(&fp)
                    .map_err(|e| miette::miette!("Failed to read: {e}"))?;
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else {
                let mut b = serde_json::json!({});
                if let Some(s) = status {
                    b["Status"] = serde_json::Value::String(s);
                }
                b
            };
            let ec = expense_claims::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&ec, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ExpenseClaimCommands::History { id } => {
            let records = expense_claims::history(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&records, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
