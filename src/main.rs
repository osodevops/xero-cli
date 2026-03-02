use clap::Parser;
use xero_cli::cli::Cli;

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
