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
use clap::Subcommand;

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
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommands,
    },
    /// Manage invoices
    Invoices {
        #[command(subcommand)]
        command: invoices::InvoiceCommands,
    },
    /// Manage contacts
    Contacts {
        #[command(subcommand)]
        command: contacts::ContactCommands,
    },
    /// Manage chart of accounts
    Accounts {
        #[command(subcommand)]
        command: accounts::AccountCommands,
    },
    /// Manage payments
    Payments {
        #[command(subcommand)]
        command: payments::PaymentCommands,
    },
    /// Manage items
    Items {
        #[command(subcommand)]
        command: items::ItemCommands,
    },
    /// Manage bank transactions
    #[command(name = "bank-transactions")]
    BankTransactions {
        #[command(subcommand)]
        command: bank_transactions::BankTransactionCommands,
    },
    /// Manage bank transfers
    #[command(name = "bank-transfers")]
    BankTransfers {
        #[command(subcommand)]
        command: bank_transfers::BankTransferCommands,
    },
    /// Manage credit notes
    #[command(name = "credit-notes")]
    CreditNotes {
        #[command(subcommand)]
        command: credit_notes::CreditNoteCommands,
    },
    /// Manage purchase orders
    #[command(name = "purchase-orders")]
    PurchaseOrders {
        #[command(subcommand)]
        command: purchase_orders::PurchaseOrderCommands,
    },
    /// Manage quotes
    Quotes {
        #[command(subcommand)]
        command: quotes::QuoteCommands,
    },
    /// List currencies
    Currencies {
        #[command(subcommand)]
        command: currencies::CurrencyCommands,
    },
    /// Manage employees
    Employees {
        #[command(subcommand)]
        command: employees::EmployeeCommands,
    },
    /// Manage users
    Users {
        #[command(subcommand)]
        command: users::UserCommands,
    },
    /// Manage budgets
    Budgets {
        #[command(subcommand)]
        command: budgets::BudgetCommands,
    },
    /// Manage branding themes
    #[command(name = "branding-themes")]
    BrandingThemes {
        #[command(subcommand)]
        command: branding_themes::BrandingThemeCommands,
    },
    /// Manage repeating invoices
    #[command(name = "repeating-invoices")]
    RepeatingInvoices {
        #[command(subcommand)]
        command: repeating_invoices::RepeatingInvoiceCommands,
    },
    /// View organisation details
    Organisation {
        #[command(subcommand)]
        command: organisation::OrganisationCommands,
    },
    /// Manage payment services
    #[command(name = "payment-services")]
    PaymentServices {
        #[command(subcommand)]
        command: payment_services::PaymentServiceCommands,
    },
    /// View financial reports
    Reports {
        #[command(subcommand)]
        command: reports::ReportCommands,
    },
    /// Manage tax rates
    #[command(name = "tax-rates")]
    TaxRates {
        #[command(subcommand)]
        command: tax_rates::TaxRateCommands,
    },
    /// Manage contact groups
    #[command(name = "contact-groups")]
    ContactGroups {
        #[command(subcommand)]
        command: contact_groups::ContactGroupCommands,
    },
    /// Manage manual journals
    #[command(name = "manual-journals")]
    ManualJournals {
        #[command(subcommand)]
        command: manual_journals::ManualJournalCommands,
    },
    /// Manage linked transactions
    #[command(name = "linked-transactions")]
    LinkedTransactions {
        #[command(subcommand)]
        command: linked_transactions::LinkedTransactionCommands,
    },
    /// Manage receipts
    Receipts {
        #[command(subcommand)]
        command: receipts::ReceiptCommands,
    },
    /// Manage batch payments
    #[command(name = "batch-payments")]
    BatchPayments {
        #[command(subcommand)]
        command: batch_payments::BatchPaymentCommands,
    },
    /// Manage expense claims
    #[command(name = "expense-claims")]
    ExpenseClaims {
        #[command(subcommand)]
        command: expense_claims::ExpenseClaimCommands,
    },
    /// Manage overpayments
    Overpayments {
        #[command(subcommand)]
        command: overpayments::OverpaymentCommands,
    },
    /// Manage prepayments
    Prepayments {
        #[command(subcommand)]
        command: prepayments::PrepaymentCommands,
    },
    /// Manage tracking categories
    #[command(name = "tracking-categories")]
    TrackingCategories {
        #[command(subcommand)]
        command: tracking_categories::TrackingCategoryCommands,
    },
    /// View journals
    Journals {
        #[command(subcommand)]
        command: journals::JournalCommands,
    },
    /// Manage response cache
    Cache {
        #[command(subcommand)]
        command: cache::CacheCommands,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: config::ConfigCommands,
    },
    /// Generate shell completions
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
