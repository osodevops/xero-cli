use crate::api::endpoints::manual_journals;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::manual_journal::ManualJournal;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ManualJournalCommands {
    /// List manual journals
    ///
    /// Retrieve all manual journals in the organisation.
    /// Manual journals are used to record adjustments or transactions
    /// that do not fit standard Xero transaction types.
    #[command(after_long_help = "\
EXAMPLES:
  xero manual-journals list
  xero manual-journals list --output json")]
    List,

    /// Get a specific manual journal
    ///
    /// Retrieve full details for a single manual journal by its UUID,
    /// including all journal lines and their account allocations.
    #[command(after_long_help = "\
EXAMPLES:
  xero manual-journals get a1b2c3d4-e5f6-7890-abcd-ef1234567890
  xero manual-journals get a1b2c3d4-e5f6-7890-abcd-ef1234567890 --output json")]
    Get {
        /// Manual journal ID (UUID)
        id: String,
    },

    /// Create a manual journal
    ///
    /// Create a new manual journal from a JSON file.
    /// The file must contain a valid manual journal payload with a narration
    /// and at least two journal lines that balance to zero.
    #[command(after_long_help = "\
EXAMPLES:
  xero manual-journals create --file journal.json
  xero manual-journals create --file adjustments/year-end.json --output json")]
    Create {
        /// Path to JSON file containing the manual journal payload
        #[arg(long)]
        file: String,
    },

    /// Update a manual journal
    ///
    /// Update an existing manual journal by providing a JSON file with the
    /// updated payload, or change just the narration inline with --narration.
    /// If --file is given it takes precedence over --narration.
    #[command(after_long_help = "\
EXAMPLES:
  xero manual-journals update a1b2c3d4-... --narration \"Corrected year-end adjustment\"
  xero manual-journals update a1b2c3d4-... --file updated-journal.json
  xero manual-journals update a1b2c3d4-... --file updated-journal.json --output json")]
    Update {
        /// Manual journal ID (UUID)
        id: String,
        /// Path to JSON file with updated journal data (takes precedence over --narration)
        #[arg(long)]
        file: Option<String>,
        /// New narration (description) for the manual journal
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
