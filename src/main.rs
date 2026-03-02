use clap::Parser;

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("XERO_GIT_HASH"), ")");

#[derive(Parser)]
#[command(name = "xero", version = VERSION, about = "A fast CLI for the Xero Accounting API")]
struct Cli {
    #[command(subcommand)]
    command: Option<xero_cli::cli::Commands>,

    /// Use a specific org profile
    #[arg(long, global = true, env = "XERO_PROFILE")]
    profile: Option<String>,

    /// Output format: table, json, csv, yaml
    #[arg(long, global = true, default_value = "table", env = "XERO_OUTPUT")]
    output: xero_cli::output::OutputFormat,

    /// Compact output (no pretty-printing)
    #[arg(long, global = true)]
    compact: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Enable verbose logging
    #[arg(long, short, global = true)]
    verbose: bool,

    /// Suppress all non-data output
    #[arg(long, global = true)]
    quiet: bool,

    /// Show the API request without executing
    #[arg(long, global = true)]
    dry_run: bool,

    /// Results per page (1-1000)
    #[arg(long, global = true, default_value = "100")]
    page_size: u32,

    /// Auto-paginate and return all results
    #[arg(long, global = true)]
    all_pages: bool,

    /// Only return records modified after this ISO8601 datetime
    #[arg(long, global = true)]
    modified_since: Option<String>,

    /// Path to config file
    #[arg(long, global = true, env = "XERO_CONFIG")]
    config: Option<String>,

    /// Disable response caching
    #[arg(long, global = true)]
    no_cache: bool,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        "xero_cli=debug"
    } else {
        "xero_cli=info"
    };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .with_target(false)
        .init();

    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            println!();
            return Ok(());
        }
    };

    let global = xero_cli::cli::GlobalArgs {
        profile: cli.profile,
        output: cli.output,
        compact: cli.compact,
        no_color: cli.no_color,
        verbose: cli.verbose,
        quiet: cli.quiet,
        dry_run: cli.dry_run,
        page_size: cli.page_size,
        all_pages: cli.all_pages,
        modified_since: cli.modified_since,
        config_path: cli.config,
        no_cache: cli.no_cache,
    };

    xero_cli::cli::dispatch(command, &global).await
}
