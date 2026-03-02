use crate::api::endpoints::reports;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::report::Report;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ReportCommands {
    /// Profit and Loss report
    ///
    /// Generates a Profit and Loss (income statement) report for the specified
    /// date range. If no dates are provided, Xero returns the current financial
    /// year to date. Rows are grouped by income and expense categories.
    #[command(after_long_help = "\
EXAMPLES:
  # Current financial year to date
  xero reports profit-and-loss

  # Specific quarter
  xero reports profit-and-loss --from 2025-01-01 --to 2025-03-31

  # Output as JSON
  xero reports profit-and-loss --from 2025-01-01 --to 2025-12-31 -o json")]
    ProfitAndLoss {
        /// Start date for the reporting period (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date for the reporting period (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Balance Sheet report
    ///
    /// Generates a Balance Sheet report showing assets, liabilities, and equity
    /// as at the specified date. Defaults to today when no date is given.
    #[command(after_long_help = "\
EXAMPLES:
  # Balance sheet as at today
  xero reports balance-sheet

  # Balance sheet at end of last quarter
  xero reports balance-sheet --date 2025-03-31

  # Output as JSON
  xero reports balance-sheet --date 2025-03-31 -o json")]
    BalanceSheet {
        /// Report as-at date (YYYY-MM-DD); defaults to today
        #[arg(long)]
        date: Option<String>,
    },
    /// Trial Balance report
    ///
    /// Generates a Trial Balance report listing all accounts with their debit
    /// and credit balances as at the specified date. Useful for verifying that
    /// total debits equal total credits.
    #[command(after_long_help = "\
EXAMPLES:
  # Trial balance as at today
  xero reports trial-balance

  # Trial balance at a specific date
  xero reports trial-balance --date 2025-06-30")]
    TrialBalance {
        /// Report as-at date (YYYY-MM-DD); defaults to today
        #[arg(long)]
        date: Option<String>,
    },
    /// Bank Summary report
    ///
    /// Retrieves a summary of all bank accounts showing opening balance,
    /// cash received, cash spent, and closing balance. Covers the current
    /// statement period configured in Xero.
    #[command(after_long_help = "\
EXAMPLES:
  # Bank summary for the current period
  xero reports bank-summary

  # Output as JSON for scripting
  xero reports bank-summary -o json")]
    BankSummary,
    /// Budget Summary report
    ///
    /// Generates a Budget Summary report comparing actual figures against
    /// budgeted amounts for the specified date range. If no dates are provided,
    /// Xero returns the current financial year.
    #[command(after_long_help = "\
EXAMPLES:
  # Budget summary for current financial year
  xero reports budget-summary

  # Budget summary for a specific period
  xero reports budget-summary --from 2025-01-01 --to 2025-06-30")]
    BudgetSummary {
        /// Start date for the budget period (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date for the budget period (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Executive Summary report
    ///
    /// Retrieves a high-level executive summary with key financial metrics
    /// including cash position, income, expenses, debtors, and creditors.
    /// No parameters are required.
    #[command(after_long_help = "\
EXAMPLES:
  # Executive summary
  xero reports executive-summary

  # Output as JSON
  xero reports executive-summary -o json")]
    ExecutiveSummary,
    /// Aged Receivables by Contact
    ///
    /// Generates an Aged Receivables report for a single contact, showing
    /// outstanding invoices grouped into aging buckets (current, 30, 60, 90+
    /// days). Useful for chasing overdue payments.
    #[command(after_long_help = "\
EXAMPLES:
  # Aged receivables for a specific contact
  xero reports aged-receivables --contact 00000000-0000-0000-0000-000000000000

  # Output as JSON
  xero reports aged-receivables --contact <CONTACT_ID> -o json")]
    AgedReceivables {
        /// Xero contact UUID to generate the report for
        #[arg(long)]
        contact: String,
    },
    /// Aged Payables by Contact
    ///
    /// Generates an Aged Payables report for a single contact, showing
    /// outstanding bills grouped into aging buckets (current, 30, 60, 90+
    /// days). Helps track what you owe to suppliers.
    #[command(after_long_help = "\
EXAMPLES:
  # Aged payables for a specific contact
  xero reports aged-payables --contact 00000000-0000-0000-0000-000000000000

  # Output as JSON
  xero reports aged-payables --contact <CONTACT_ID> -o json")]
    AgedPayables {
        /// Xero contact UUID to generate the report for
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
