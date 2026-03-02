use crate::api::endpoints::journals;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::journal::Journal;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum JournalCommands {
    /// List journals
    ///
    /// Retrieve system journals which record the double-entry accounting
    /// movements for every transaction. Unlike most Xero endpoints, journals
    /// use offset-based pagination rather than page numbers. Use --offset to
    /// resume from a specific position, or --all-pages to fetch everything.
    #[command(after_long_help = "\
EXAMPLES:
  xero journals list
  xero journals list --offset 100
  xero journals list --all-pages
  xero journals list --output json --offset 500")]
    List {
        /// Offset for pagination (journals use offset-based pagination, not page numbers)
        #[arg(long)]
        offset: Option<u64>,
    },

    /// Get a specific journal
    ///
    /// Retrieve full details for a single journal by its UUID, including
    /// all journal lines with their account codes, debits, and credits.
    #[command(after_long_help = "\
EXAMPLES:
  xero journals get d4e5f6a7-8b9c-0d1e-2f3a-4b5c6d7e8f9a
  xero journals get d4e5f6a7-8b9c-0d1e-2f3a-4b5c6d7e8f9a --output json")]
    Get {
        /// Journal ID (UUID)
        id: String,
    },
}

impl Tabular for Journal {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "Number".to_string(),
            "Date".to_string(),
            "Source Type".to_string(),
            "Reference".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.journal_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.journal_number
                .map(|n| n.to_string())
                .unwrap_or_default(),
            self.journal_date.clone().unwrap_or_default(),
            self.source_type.clone().unwrap_or_default(),
            self.reference.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: JournalCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        JournalCommands::List { offset } => {
            let list = if global.all_pages {
                journals::list_all(&client)
                    .await
                    .map_err(|e| miette::miette!("{e}"))?
            } else {
                journals::list(&client, offset)
                    .await
                    .map_err(|e| miette::miette!("{e}"))?
            };
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        JournalCommands::Get { id } => {
            let j = journals::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&j, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
