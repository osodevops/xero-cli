use crate::api::endpoints::currencies;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::currency::Currency;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum CurrencyCommands {
    /// List currencies
    List,
}

impl Tabular for Currency {
    fn headers() -> Vec<String> {
        vec!["Code".to_string(), "Description".to_string()]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.code.clone().unwrap_or_default(),
            self.description.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: CurrencyCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        CurrencyCommands::List => {
            let list = currencies::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
