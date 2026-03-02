use crate::api::endpoints::tax_rates;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::tax_rate::TaxRate;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TaxRateCommands {
    /// List tax rates
    List,
    /// Create a tax rate
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        tax_type: String,
        #[arg(long)]
        rate: String,
    },
    /// Update a tax rate
    Update {
        #[arg(long)]
        name: String,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        file: Option<String>,
    },
}

impl Tabular for TaxRate {
    fn headers() -> Vec<String> {
        vec!["Name".into(), "Type".into(), "Rate".into(), "Status".into()]
    }
    fn row(&self) -> Vec<String> {
        vec![
            self.name.clone().unwrap_or_default(),
            self.tax_type.clone().unwrap_or_default(),
            self.effective_rate
                .map(|r| r.to_string())
                .unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: TaxRateCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;
    match command {
        TaxRateCommands::List => {
            let list = tax_rates::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        TaxRateCommands::Create {
            name,
            tax_type,
            rate,
        } => {
            let body = serde_json::json!({"Name": name, "TaxType": tax_type, "TaxComponents": [{"Name": name, "Rate": rate}]});
            let tr = tax_rates::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&tr, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
        TaxRateCommands::Update { name, status, file } => {
            let body = if let Some(fp) = file {
                let c = std::fs::read_to_string(&fp)
                    .map_err(|e| miette::miette!("Failed to read file: {e}"))?;
                serde_json::from_str(&c).map_err(|e| miette::miette!("Invalid JSON: {e}"))?
            } else {
                let mut b = serde_json::json!({"Name": name});
                if let Some(s) = status {
                    b["Status"] = serde_json::Value::String(s);
                }
                b
            };
            let tr = tax_rates::update(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&tr, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }
    Ok(())
}
