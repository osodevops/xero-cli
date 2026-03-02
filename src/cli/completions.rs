use clap::Subcommand;
use clap_complete::{generate, Shell};

#[derive(Subcommand)]
pub enum CompletionCommands {
    /// Generate bash completions
    Bash,
    /// Generate zsh completions
    Zsh,
    /// Generate fish completions
    Fish,
}

pub fn execute(command: CompletionCommands) -> miette::Result<()> {
    let shell = match command {
        CompletionCommands::Bash => Shell::Bash,
        CompletionCommands::Zsh => Shell::Zsh,
        CompletionCommands::Fish => Shell::Fish,
    };

    // We need to build a clap Command to generate completions
    // Since we can't easily get the full command here, we create a minimal one
    let mut cmd = build_cli_command();
    generate(shell, &mut cmd, "xero", &mut std::io::stdout());

    Ok(())
}

fn build_cli_command() -> clap::Command {
    use clap::{Arg, Command};

    Command::new("xero")
        .about("A fast CLI for the Xero Accounting API")
        .subcommand(
            Command::new("auth")
                .about("Manage authentication")
                .subcommand(Command::new("login").about("Interactive PKCE login"))
                .subcommand(Command::new("status").about("Show auth status"))
                .subcommand(Command::new("refresh").about("Force token refresh"))
                .subcommand(Command::new("logout").about("Clear stored tokens"))
                .subcommand(Command::new("setup-m2m").about("Configure client credentials"))
                .subcommand(Command::new("scopes").about("Manage OAuth scopes")),
        )
        .subcommand(
            Command::new("invoices")
                .about("Manage invoices")
                .subcommand(Command::new("list").about("List invoices"))
                .subcommand(Command::new("get").about("Get invoice").arg(Arg::new("id")))
                .subcommand(Command::new("create").about("Create invoice"))
                .subcommand(Command::new("update").about("Update invoice")),
        )
        .subcommand(
            Command::new("contacts")
                .about("Manage contacts")
                .subcommand(Command::new("list").about("List contacts"))
                .subcommand(Command::new("get").about("Get contact").arg(Arg::new("id")))
                .subcommand(Command::new("create").about("Create contact"))
                .subcommand(Command::new("update").about("Update contact")),
        )
        .subcommand(
            Command::new("accounts")
                .about("Manage accounts")
                .subcommand(Command::new("list").about("List accounts"))
                .subcommand(Command::new("get").about("Get account").arg(Arg::new("id")))
                .subcommand(Command::new("create").about("Create account"))
                .subcommand(Command::new("archive").about("Archive account")),
        )
        .subcommand(
            Command::new("reports")
                .about("View financial reports")
                .subcommand(Command::new("profit-and-loss").about("P&L report"))
                .subcommand(Command::new("balance-sheet").about("Balance sheet"))
                .subcommand(Command::new("trial-balance").about("Trial balance"))
                .subcommand(Command::new("bank-summary").about("Bank summary"))
                .subcommand(Command::new("budget-summary").about("Budget summary"))
                .subcommand(Command::new("executive-summary").about("Executive summary"))
                .subcommand(Command::new("aged-receivables").about("Aged receivables"))
                .subcommand(Command::new("aged-payables").about("Aged payables")),
        )
        .subcommand(
            Command::new("config")
                .about("Manage configuration")
                .subcommand(Command::new("init").about("Initialize config"))
                .subcommand(Command::new("show").about("Show config"))
                .subcommand(Command::new("set").about("Set config value")),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .subcommand(Command::new("bash").about("Bash completions"))
                .subcommand(Command::new("zsh").about("Zsh completions"))
                .subcommand(Command::new("fish").about("Fish completions")),
        )
}
