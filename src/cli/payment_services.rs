use crate::api::endpoints::payment_services;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::payment_service::PaymentService;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum PaymentServiceCommands {
    /// List payment services
    List,
}

impl Tabular for PaymentService {
    fn headers() -> Vec<String> {
        vec!["ID".to_string(), "Name".to_string(), "Type".to_string()]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.payment_service_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.payment_service_name.clone().unwrap_or_default(),
            self.payment_service_type.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: PaymentServiceCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        PaymentServiceCommands::List => {
            let list = payment_services::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
