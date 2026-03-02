use clap::{CommandFactory, Subcommand};
use clap_complete::{generate, Shell};

#[derive(Subcommand)]
pub enum CompletionCommands {
    /// Generate bash completions
    ///
    /// Print bash completion script to stdout.
    /// Add to your shell profile: eval "$(xero completions bash)"
    #[command(after_long_help = "\
EXAMPLES:
  xero completions bash > ~/.local/share/bash-completion/completions/xero
  eval \"$(xero completions bash)\"")]
    Bash,

    /// Generate zsh completions
    ///
    /// Print zsh completion script to stdout.
    /// Save to a file in your $fpath.
    #[command(after_long_help = "\
EXAMPLES:
  xero completions zsh > ~/.zfunc/_xero
  xero completions zsh > \"${fpath[1]}/_xero\"")]
    Zsh,

    /// Generate fish completions
    ///
    /// Print fish completion script to stdout.
    #[command(after_long_help = "\
EXAMPLES:
  xero completions fish > ~/.config/fish/completions/xero.fish")]
    Fish,

    /// Generate man pages
    ///
    /// Generate man page files for xero and all subcommands.
    /// Creates individual .1 files (e.g. xero.1, xero-invoices.1, xero-invoices-list.1).
    #[command(after_long_help = "\
EXAMPLES:
  xero completions man --output-dir /usr/local/share/man/man1
  xero completions man --output-dir ./man
  man ./man/xero.1
  man ./man/xero-invoices-list.1")]
    Man {
        /// Directory to write man page files into (created if it does not exist)
        #[arg(long)]
        output_dir: String,
    },
}

pub fn execute(command: CompletionCommands) -> miette::Result<()> {
    match command {
        CompletionCommands::Bash | CompletionCommands::Zsh | CompletionCommands::Fish => {
            let shell = match command {
                CompletionCommands::Bash => Shell::Bash,
                CompletionCommands::Zsh => Shell::Zsh,
                CompletionCommands::Fish => Shell::Fish,
                _ => unreachable!(),
            };

            let mut cmd = crate::cli::Cli::command();
            generate(shell, &mut cmd, "xero", &mut std::io::stdout());
        }
        CompletionCommands::Man { output_dir } => {
            let cmd = crate::cli::Cli::command();
            let out = std::path::PathBuf::from(&output_dir);
            std::fs::create_dir_all(&out)
                .map_err(|e| miette::miette!("Failed to create output directory: {e}"))?;
            generate_man_pages(&cmd, &out, "xero")?;
            eprintln!("Man pages written to {output_dir}");
        }
    }

    Ok(())
}

fn generate_man_pages(
    cmd: &clap::Command,
    out_dir: &std::path::Path,
    prefix: &str,
) -> miette::Result<()> {
    let man = clap_mangen::Man::new(cmd.clone());
    let filename = format!("{prefix}.1");
    let path = out_dir.join(&filename);
    let mut buf = Vec::new();
    man.render(&mut buf)
        .map_err(|e| miette::miette!("Failed to render man page for {prefix}: {e}"))?;
    std::fs::write(&path, buf)
        .map_err(|e| miette::miette!("Failed to write {}: {e}", path.display()))?;

    for sub in cmd.get_subcommands() {
        if sub.get_name() == "help" {
            continue;
        }
        let sub_prefix = format!("{prefix}-{}", sub.get_name());
        generate_man_pages(sub, out_dir, &sub_prefix)?;
    }

    Ok(())
}
