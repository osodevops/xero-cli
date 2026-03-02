use crate::api::endpoints::manual_journals;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::manual_journal::ManualJournal;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ManualJournalCommands {
    /// List manual journals
    List,
    /// Get a specific manual journal
    Get { id: String },
    /// Create a manual journal
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a manual journal
    Update {
        id: String,
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        narration: Option<String>,
    },
}

impl Tabular for ManualJournal {
    fn headers() -> Vec<String> {
        vec![
            "ID".into(),
            "Narration".into(),
            "Status".into(),
            "Date".into(),
            "Lines".into(),
        ]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.manual_journal_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.narration.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
            self.journal_lines.len().to_string(),
        ]
    }
}

pub async fn execute(command: ManualJournalCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        ManualJournalCommands::List => {
            let list = manual_journals::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ManualJournalCommands::Get { id } => {
            let mj = manual_journals::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&mj, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ManualJournalCommands::Create { file } => {
            let content = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
            let body: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| miette::miette!("Invalid JSON: {e}"))?;
            let mj = manual_journals::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&mj, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        ManualJournalCommands::Update {
            id,
            file,
            narration,
        } => {
            let body = if let Some(fp) = file {
                let c = std::fs::read_to_string(&fp)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else {
                let mut b = serde_json::json!({});
                if let Some(n) = narration {
                    b["Narration"] = serde_json::Value::String(n);
                }
                b
            };
            let mj = manual_journals::update(&client, &id, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&mj, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
