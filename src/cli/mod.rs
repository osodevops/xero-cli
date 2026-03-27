pub mod accounts;
pub mod auth;
pub mod bank_transactions;
pub mod bank_transfers;
pub mod batch_payments;
pub mod branding_themes;
pub mod budgets;
pub mod cache;
pub mod common;
pub mod completions;
pub mod config;
pub mod contact_groups;
pub mod contacts;
pub mod credit_notes;
pub mod currencies;
pub mod employees;
pub mod expense_claims;
pub mod invoices;
pub mod items;
pub mod journals;
pub mod linked_transactions;
pub mod manual_journals;
pub mod organisation;
pub mod overpayments;
pub mod payment_services;
pub mod payments;
pub mod prepayments;
pub mod purchase_orders;
pub mod quotes;
pub mod receipts;
pub mod repeating_invoices;
pub mod reports;
pub mod tax_rates;
pub mod tracking_categories;
pub mod users;

use crate::output::OutputFormat;
use clap::{Parser, Subcommand};

/// A fast CLI for the Xero Accounting API
///
/// xero-cli provides full coverage of the Xero Accounting API with support for
/// all 34 resource types including invoices, contacts, accounts, payments,
/// bank transactions, reports, and more.
///
/// GETTING STARTED:
///   1. Register an app at https://developer.xero.com/app/manage
///   2. Run `xero config init` to create a configuration file
///   3. Run `xero auth login` to authenticate via OAuth2 PKCE flow
///   4. Run `xero invoices list` to verify everything works
///
/// ENVIRONMENT VARIABLES:
///   XERO_CLIENT_ID      OAuth2 client ID (overrides config file)
///   XERO_CLIENT_SECRET   OAuth2 client secret (for M2M auth)
///   XERO_ACCESS_TOKEN    Direct bearer token (skips OAuth, useful for CI)
///   XERO_TENANT_ID       Xero organisation tenant ID
///   XERO_PROFILE         Named org profile to use
///   XERO_OUTPUT          Default output format (table, json, csv, yaml)
///   XERO_CONFIG          Path to config file
#[derive(Parser)]
#[command(
    name = "xero",
    version = crate::VERSION,
    after_long_help = "\
EXAMPLES:
  xero invoices list --status AUTHORISED
  xero invoices list --output json --all-pages
  xero contacts list --search \"Acme\" --output csv
  xero invoices create --contact \"Acme Corp\" --line-item \"Consulting,10,150.00\"
  xero reports profit-and-loss --from 2024-01-01 --to 2024-12-31
  xero payments create --invoice INV-0001 --account 090 --amount 250.00
  xero auth status"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Use a specific org profile
    #[arg(long, global = true, env = "XERO_PROFILE")]
    pub profile: Option<String>,

    /// Output format: table, json, csv, yaml
    #[arg(long, global = true, default_value = "table", env = "XERO_OUTPUT")]
    pub output: OutputFormat,

    /// Compact output (no pretty-printing)
    #[arg(long, global = true)]
    pub compact: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Enable verbose logging
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Suppress all non-data output
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Show the API request without executing
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Results per page (1-1000)
    #[arg(long, global = true, default_value = "100")]
    pub page_size: u32,

    /// Auto-paginate and return all results
    #[arg(long, global = true)]
    pub all_pages: bool,

    /// Only return records modified after this ISO8601 datetime
    #[arg(long, global = true)]
    pub modified_since: Option<String>,

    /// Path to config file
    #[arg(long, global = true, env = "XERO_CONFIG")]
    pub config: Option<String>,

    /// Disable response caching
    #[arg(long, global = true)]
    pub no_cache: bool,
}

#[derive(Debug, Clone)]
pub struct GlobalArgs {
    pub profile: Option<String>,
    pub output: OutputFormat,
    pub compact: bool,
    pub no_color: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub dry_run: bool,
    pub page_size: u32,
    pub all_pages: bool,
    pub modified_since: Option<String>,
    pub config_path: Option<String>,
    pub no_cache: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage authentication
    ///
    /// Login, logout, check status, and manage OAuth scopes.
    /// Supports both interactive PKCE login and machine-to-machine (M2M)
    /// client credentials flow.
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommands,
    },

    /// Manage invoices
    ///
    /// List, get, create, and update sales invoices (ACCREC) and bills (ACCPAY).
    /// Supports filtering by status, contact, date range, and custom where clauses.
    /// Requires OAuth scope: accounting.invoices or accounting.invoices.read.
    Invoices {
        #[command(subcommand)]
        command: invoices::InvoiceCommands,
    },

    /// Manage contacts
    ///
    /// List, get, create, and update customers and suppliers.
    /// Contacts are shared across invoices, bills, payments, and bank transactions.
    /// Requires OAuth scope: accounting.contacts or accounting.contacts.read.
    Contacts {
        #[command(subcommand)]
        command: contacts::ContactCommands,
    },

    /// Manage chart of accounts
    ///
    /// List, get, create, and archive accounts in the chart of accounts.
    /// Account types: BANK, CURRENT, CURRLIAB, DEPRECIATN, DIRECTCOSTS, EQUITY,
    /// EXPENSE, FIXED, INVENTORY, LIABILITY, NONCURRENT, OTHERINCOME, OVERHEADS,
    /// PREPAYMENT, REVENUE, SALES, TERMLIAB, PAYGLIABILITY, SUPERANNUATIONEXPENSE,
    /// SUPERANNUATIONLIABILITY, WAGESEXPENSE.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    Accounts {
        #[command(subcommand)]
        command: accounts::AccountCommands,
    },

    /// Manage payments
    ///
    /// List, get, create, and delete payments against invoices and credit notes.
    /// View payment history for audit trails.
    /// Requires OAuth scope: accounting.payments or accounting.payments.read.
    Payments {
        #[command(subcommand)]
        command: payments::PaymentCommands,
    },

    /// Manage items
    ///
    /// List, get, create, update, and delete inventory items and service items.
    /// Items can be attached to invoice line items for consistent pricing.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    Items {
        #[command(subcommand)]
        command: items::ItemCommands,
    },

    /// Manage bank transactions
    ///
    /// List, get, create, and delete spend and receive money transactions.
    /// These are transactions that appear on bank statements.
    /// Requires OAuth scope: accounting.banktransactions or accounting.banktransactions.read.
    #[command(name = "bank-transactions")]
    BankTransactions {
        #[command(subcommand)]
        command: bank_transactions::BankTransactionCommands,
    },

    /// Manage bank transfers
    ///
    /// List, get, and create transfers between bank accounts.
    /// Requires OAuth scope: accounting.banktransactions or accounting.banktransactions.read.
    #[command(name = "bank-transfers")]
    BankTransfers {
        #[command(subcommand)]
        command: bank_transfers::BankTransferCommands,
    },

    /// Manage credit notes
    ///
    /// List, get, create, and allocate credit notes against invoices.
    /// Credit notes reduce the amount owing on an invoice.
    /// Requires OAuth scope: accounting.invoices or accounting.invoices.read.
    #[command(name = "credit-notes")]
    CreditNotes {
        #[command(subcommand)]
        command: credit_notes::CreditNoteCommands,
    },

    /// Manage purchase orders
    ///
    /// List, get, and create purchase orders for supplier orders.
    /// Status values: DRAFT, SUBMITTED, AUTHORISED, BILLED, DELETED.
    /// Requires OAuth scope: accounting.invoices or accounting.invoices.read.
    #[command(name = "purchase-orders")]
    PurchaseOrders {
        #[command(subcommand)]
        command: purchase_orders::PurchaseOrderCommands,
    },

    /// Manage quotes
    ///
    /// List, get, create, and update quotes (estimates).
    /// Status values: DRAFT, SENT, ACCEPTED, INVOICED, DECLINED, DELETED.
    /// Requires OAuth scope: accounting.invoices or accounting.invoices.read.
    Quotes {
        #[command(subcommand)]
        command: quotes::QuoteCommands,
    },

    /// List currencies
    ///
    /// View all currencies configured for the organisation.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    Currencies {
        #[command(subcommand)]
        command: currencies::CurrencyCommands,
    },

    /// Manage employees
    ///
    /// List and view employees in the organisation.
    /// Note: this is the Accounting API employee endpoint, not Payroll.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    Employees {
        #[command(subcommand)]
        command: employees::EmployeeCommands,
    },

    /// Manage users
    ///
    /// List and view users who have access to the Xero organisation.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    Users {
        #[command(subcommand)]
        command: users::UserCommands,
    },

    /// Manage budgets
    ///
    /// List and view budgets for the organisation.
    /// Requires OAuth scope: accounting.budgets.read.
    Budgets {
        #[command(subcommand)]
        command: budgets::BudgetCommands,
    },

    /// Manage branding themes
    ///
    /// List and view branding themes used for invoice and quote templates.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    #[command(name = "branding-themes")]
    BrandingThemes {
        #[command(subcommand)]
        command: branding_themes::BrandingThemeCommands,
    },

    /// Manage repeating invoices
    ///
    /// List and view recurring invoice templates.
    /// Requires OAuth scope: accounting.invoices or accounting.invoices.read.
    #[command(name = "repeating-invoices")]
    RepeatingInvoices {
        #[command(subcommand)]
        command: repeating_invoices::RepeatingInvoiceCommands,
    },

    /// View organisation details
    ///
    /// Retrieve organisation name, address, financial year end, tax registration,
    /// base currency, and other settings.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    Organisation {
        #[command(subcommand)]
        command: organisation::OrganisationCommands,
    },

    /// Manage payment services
    ///
    /// List online payment services configured for the organisation.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    #[command(name = "payment-services")]
    PaymentServices {
        #[command(subcommand)]
        command: payment_services::PaymentServiceCommands,
    },

    /// View financial reports
    ///
    /// Generate Profit & Loss, Balance Sheet, Trial Balance, Bank Summary,
    /// Budget Summary, Executive Summary, Aged Receivables, and Aged Payables reports.
    /// Requires granular report scopes (e.g. accounting.reports.profitandloss.read).
    Reports {
        #[command(subcommand)]
        command: reports::ReportCommands,
    },

    /// Manage tax rates
    ///
    /// List, create, and update tax rates for the organisation.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    #[command(name = "tax-rates")]
    TaxRates {
        #[command(subcommand)]
        command: tax_rates::TaxRateCommands,
    },

    /// Manage contact groups
    ///
    /// List, get, create, update, and delete contact groups.
    /// Contact groups allow you to organise contacts into categories.
    /// Requires OAuth scope: accounting.contacts or accounting.contacts.read.
    #[command(name = "contact-groups")]
    ContactGroups {
        #[command(subcommand)]
        command: contact_groups::ContactGroupCommands,
    },

    /// Manage manual journals
    ///
    /// List, get, create, and update manual journal entries.
    /// Manual journals must have at least two lines that balance to zero.
    /// Requires OAuth scope: accounting.manualjournals or accounting.manualjournals.read.
    #[command(name = "manual-journals")]
    ManualJournals {
        #[command(subcommand)]
        command: manual_journals::ManualJournalCommands,
    },

    /// Manage linked transactions
    ///
    /// List, get, create, update, and delete linked transactions.
    /// Linked transactions connect billable expenses to customer invoices.
    /// Requires OAuth scope: accounting.invoices or accounting.invoices.read.
    #[command(name = "linked-transactions")]
    LinkedTransactions {
        #[command(subcommand)]
        command: linked_transactions::LinkedTransactionCommands,
    },

    /// Manage receipts
    ///
    /// List, get, and create expense receipts.
    /// Receipts are submitted by users and attached to expense claims.
    /// Requires OAuth scope: accounting.classicexpenses or accounting.classicexpenses.read.
    Receipts {
        #[command(subcommand)]
        command: receipts::ReceiptCommands,
    },

    /// Manage batch payments
    ///
    /// List, get, create, and delete batch payments.
    /// Batch payments allow multiple invoices to be paid in a single transaction.
    /// Requires OAuth scope: accounting.payments or accounting.payments.read.
    #[command(name = "batch-payments")]
    BatchPayments {
        #[command(subcommand)]
        command: batch_payments::BatchPaymentCommands,
    },

    /// Manage expense claims
    ///
    /// List, get, create, and update expense claims.
    /// Expense claims group receipts for reimbursement.
    /// Status values: SUBMITTED, AUTHORISED, PAID, VOIDED.
    /// Requires OAuth scope: accounting.classicexpenses or accounting.classicexpenses.read.
    #[command(name = "expense-claims")]
    ExpenseClaims {
        #[command(subcommand)]
        command: expense_claims::ExpenseClaimCommands,
    },

    /// Manage overpayments
    ///
    /// List, get, and allocate overpayments to invoices.
    /// Overpayments occur when a payment exceeds the invoice amount.
    /// Requires OAuth scope: accounting.payments or accounting.payments.read.
    Overpayments {
        #[command(subcommand)]
        command: overpayments::OverpaymentCommands,
    },

    /// Manage prepayments
    ///
    /// List, get, and allocate prepayments to invoices.
    /// Prepayments are advance payments before an invoice is raised.
    /// Requires OAuth scope: accounting.payments or accounting.payments.read.
    Prepayments {
        #[command(subcommand)]
        command: prepayments::PrepaymentCommands,
    },

    /// Manage tracking categories
    ///
    /// List, get, create, and update tracking categories and their options.
    /// Tracking categories allow you to assign additional dimensions (e.g. Region,
    /// Department) to transactions for reporting.
    /// Requires OAuth scope: accounting.settings or accounting.settings.read.
    #[command(name = "tracking-categories")]
    TrackingCategories {
        #[command(subcommand)]
        command: tracking_categories::TrackingCategoryCommands,
    },

    /// View journals
    ///
    /// List and view system-generated journal entries.
    /// Journals are automatically created by Xero for every accounting transaction.
    /// Uses offset-based pagination (not page-based).
    /// Requires OAuth scope: accounting.journals.read.
    Journals {
        #[command(subcommand)]
        command: journals::JournalCommands,
    },

    /// Manage response cache
    ///
    /// Clear cached API responses or view cache statistics.
    /// The cache stores GET responses locally to reduce API calls
    /// and improve performance.
    Cache {
        #[command(subcommand)]
        command: cache::CacheCommands,
    },

    /// Manage configuration
    ///
    /// Initialize, view, and modify the xero-cli configuration file.
    /// Configuration is stored at ~/.config/xero-cli/config.toml by default.
    Config {
        #[command(subcommand)]
        command: config::ConfigCommands,
    },

    /// Generate shell completions and man pages
    ///
    /// Generate tab-completion scripts for bash, zsh, or fish shells,
    /// or generate man pages for offline reference.
    Completions {
        #[command(subcommand)]
        command: completions::CompletionCommands,
    },
}

pub async fn dispatch(command: Commands, global: &GlobalArgs) -> miette::Result<()> {
    match command {
        Commands::Auth { command } => auth::execute(command, global).await,
        Commands::Invoices { command } => invoices::execute(command, global).await,
        Commands::Contacts { command } => contacts::execute(command, global).await,
        Commands::Accounts { command } => accounts::execute(command, global).await,
        Commands::Payments { command } => payments::execute(command, global).await,
        Commands::Items { command } => items::execute(command, global).await,
        Commands::BankTransactions { command } => bank_transactions::execute(command, global).await,
        Commands::BankTransfers { command } => bank_transfers::execute(command, global).await,
        Commands::CreditNotes { command } => credit_notes::execute(command, global).await,
        Commands::PurchaseOrders { command } => purchase_orders::execute(command, global).await,
        Commands::Quotes { command } => quotes::execute(command, global).await,
        Commands::Currencies { command } => currencies::execute(command, global).await,
        Commands::Employees { command } => employees::execute(command, global).await,
        Commands::Users { command } => users::execute(command, global).await,
        Commands::Budgets { command } => budgets::execute(command, global).await,
        Commands::BrandingThemes { command } => branding_themes::execute(command, global).await,
        Commands::RepeatingInvoices { command } => {
            repeating_invoices::execute(command, global).await
        }
        Commands::Organisation { command } => organisation::execute(command, global).await,
        Commands::PaymentServices { command } => payment_services::execute(command, global).await,
        Commands::Reports { command } => reports::execute(command, global).await,
        Commands::TaxRates { command } => tax_rates::execute(command, global).await,
        Commands::ContactGroups { command } => contact_groups::execute(command, global).await,
        Commands::ManualJournals { command } => manual_journals::execute(command, global).await,
        Commands::LinkedTransactions { command } => {
            linked_transactions::execute(command, global).await
        }
        Commands::Receipts { command } => receipts::execute(command, global).await,
        Commands::BatchPayments { command } => batch_payments::execute(command, global).await,
        Commands::ExpenseClaims { command } => expense_claims::execute(command, global).await,
        Commands::Overpayments { command } => overpayments::execute(command, global).await,
        Commands::Prepayments { command } => prepayments::execute(command, global).await,
        Commands::TrackingCategories { command } => {
            tracking_categories::execute(command, global).await
        }
        Commands::Journals { command } => journals::execute(command, global).await,
        Commands::Cache { command } => cache::execute(command, global).await,
        Commands::Config { command } => config::execute(command, global).await,
        Commands::Completions { command } => completions::execute(command),
    }
}
