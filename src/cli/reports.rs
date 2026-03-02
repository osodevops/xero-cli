use crate::api::endpoints::reports;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::report::Report;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ReportCommands {
    /// Profit and Loss report
    ProfitAndLoss {
        /// From date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// To date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Balance Sheet report
    BalanceSheet {
        /// Report date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
    /// Trial Balance report
    TrialBalance {
        /// Report date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
    /// Bank Summary report
    BankSummary,
    /// Budget Summary report
    BudgetSummary {
        /// From date
        #[arg(long)]
        from: Option<String>,
        /// To date
        #[arg(long)]
        to: Option<String>,
    },
    /// Executive Summary report
    ExecutiveSummary,
    /// Aged Receivables by Contact
    AgedReceivables {
        /// Contact ID
        #[arg(long)]
        contact: String,
    },
    /// Aged Payables by Contact
    AgedPayables {
        /// Contact ID
        #[arg(long)]
        contact: String,
    },
}

// Reports have a nested structure, so we flatten rows for table display
impl Tabular for ReportRowFlat {
    fn headers() -> Vec<String> {
        vec![
            "Section".to_string(),
            "Account".to_string(),
            "Value".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.section.clone(),
            self.account.clone(),
            self.value.clone(),
        ]
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct ReportRowFlat {
    section: String,
    account: String,
    value: String,
}

fn flatten_report(report: &Report) -> Vec<ReportRowFlat> {
    let mut flat = Vec::new();
    for row in &report.rows {
        let section = row.title.clone().unwrap_or_default();
        if !row.rows.is_empty() {
            for sub_row in &row.rows {
                let cells = &sub_row.cells;
                let account = cells
                    .first()
                    .and_then(|c| c.value.clone())
                    .unwrap_or_default();
                let value = cells
                    .get(1)
                    .and_then(|c| c.value.clone())
                    .unwrap_or_default();
                flat.push(ReportRowFlat {
                    section: section.clone(),
                    account,
                    value,
                });
            }
        } else if !row.cells.is_empty() {
            let account = row
                .cells
                .first()
                .and_then(|c| c.value.clone())
                .unwrap_or_default();
            let value = row
                .cells
                .get(1)
                .and_then(|c| c.value.clone())
                .unwrap_or_default();
            flat.push(ReportRowFlat {
                section: section.clone(),
                account,
                value,
            });
        }
    }
    flat
}

pub async fn execute(command: ReportCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    let report = match command {
        ReportCommands::ProfitAndLoss { from, to } => {
            reports::profit_and_loss(&client, from.as_deref(), to.as_deref())
                .await
                .map_err(|e| miette::miette!("{e}"))?
        }
        ReportCommands::BalanceSheet { date } => reports::balance_sheet(&client, date.as_deref())
            .await
            .map_err(|e| miette::miette!("{e}"))?,
        ReportCommands::TrialBalance { date } => reports::trial_balance(&client, date.as_deref())
            .await
            .map_err(|e| miette::miette!("{e}"))?,
        ReportCommands::BankSummary => reports::bank_summary(&client)
            .await
            .map_err(|e| miette::miette!("{e}"))?,
        ReportCommands::BudgetSummary { from, to } => {
            reports::budget_summary(&client, from.as_deref(), to.as_deref())
                .await
                .map_err(|e| miette::miette!("{e}"))?
        }
        ReportCommands::ExecutiveSummary => reports::executive_summary(&client)
            .await
            .map_err(|e| miette::miette!("{e}"))?,
        ReportCommands::AgedReceivables { contact } => reports::aged_receivables(&client, &contact)
            .await
            .map_err(|e| miette::miette!("{e}"))?,
        ReportCommands::AgedPayables { contact } => reports::aged_payables(&client, &contact)
            .await
            .map_err(|e| miette::miette!("{e}"))?,
    };

    if let Some(name) = &report.report_name {
        if !global.quiet {
            eprintln!("Report: {name}");
        }
    }

    match global.output {
        crate::output::OutputFormat::Table => {
            let flat = flatten_report(&report);
            let rendered = output::render(&flat, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        _ => {
            let rendered = output::render_single(&report, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
