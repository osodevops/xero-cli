use crate::api::endpoints::tax_rates;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::tax_rate::TaxRate;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TaxRateCommands {
    /// List tax rates
    ///
    /// Retrieves all tax rates configured in your Xero organisation, including
    /// the tax name, type, effective rate, and current status (ACTIVE, DELETED,
    /// etc.).
    #[command(after_long_help = "\
EXAMPLES:
  # List all tax rates
  xero tax-rates list

  # Output as JSON
  xero tax-rates list -o json

  # Compact table output
  xero tax-rates list --compact")]
    List,
    /// Create a tax rate
    ///
    /// Creates a new tax rate in Xero with the given name, tax type, and
    /// percentage rate. A single tax component is created using the provided
    /// name and rate.
    #[command(after_long_help = "\
EXAMPLES:
  # Create a 15% output tax rate
  xero tax-rates create --name \"GST on Income\" --tax-type OUTPUT --rate 15

  # Create a 20% VAT rate
  xero tax-rates create --name \"Standard VAT\" --tax-type OUTPUT --rate 20")]
    Create {
        /// Display name for the new tax rate
        #[arg(long)]
        name: String,
        /// Tax type (e.g. OUTPUT, INPUT, EXEMPTOUTPUT)
        #[arg(long)]
        tax_type: String,
        /// Tax percentage rate (e.g. 15, 20)
        #[arg(long)]
        rate: String,
    },
    /// Update a tax rate
    ///
    /// Updates an existing tax rate identified by name. You can change the
    /// status inline with --status, or supply a JSON file containing the full
    /// tax-rate payload with --file.
    #[command(after_long_help = "\
EXAMPLES:
  # Deactivate a tax rate
  xero tax-rates update --name \"Old Tax\" --status DELETED

  # Re-activate a tax rate
  xero tax-rates update --name \"Old Tax\" --status ACTIVE

  # Update from a JSON file
  xero tax-rates update --name \"Custom Tax\" --file tax_rate.json")]
    Update {
        /// Name of the tax rate to update (must match an existing rate)
        #[arg(long)]
        name: String,
        /// New status for the tax rate (e.g. ACTIVE, DELETED)
        #[arg(long)]
        status: Option<String>,
        /// Path to a JSON file containing the full tax-rate update payload
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
