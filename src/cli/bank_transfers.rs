use crate::api::endpoints::bank_transfers;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::bank_transfer::BankTransfer;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BankTransferCommands {
    /// List bank transfers
    List,
    /// Get a specific bank transfer
    Get {
        /// Bank transfer ID
        id: String,
    },
    /// Create a bank transfer
    Create {
        /// Source bank account ID
        #[arg(long)]
        from_account: String,
        /// Destination bank account ID
        #[arg(long)]
        to_account: String,
        /// Transfer amount
        #[arg(long)]
        amount: String,
    },
}

impl Tabular for BankTransfer {
    fn headers() -> Vec<String> {
        vec![
            "ID".to_string(),
            "From".to_string(),
            "To".to_string(),
            "Amount".to_string(),
            "Date".to_string(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.bank_transfer_id
                .as_deref()
                .unwrap_or_default()
                .chars()
                .take(8)
                .collect(),
            self.from_bank_account
                .as_ref()
                .and_then(|a| a.name.clone())
                .unwrap_or_default(),
            self.to_bank_account
                .as_ref()
                .and_then(|a| a.name.clone())
                .unwrap_or_default(),
            self.amount.map(|a| a.to_string()).unwrap_or_default(),
            self.date.clone().unwrap_or_default(),
        ]
    }
}

pub async fn execute(command: BankTransferCommands, global: &GlobalArgs) -> miette::Result<()> {
    let client = build_client(global)
        .await
        .map_err(|e| miette::miette!("{e}"))?;

    match command {
        BankTransferCommands::List => {
            let list = bank_transfers::list(&client)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render(&list, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        BankTransferCommands::Get { id } => {
            let bt = bank_transfers::get(&client, &id)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&bt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }

        BankTransferCommands::Create {
            from_account,
            to_account,
            amount,
        } => {
            let body = serde_json::json!({
                "FromBankAccount": {"AccountID": from_account},
                "ToBankAccount": {"AccountID": to_account},
                "Amount": amount,
            });
            let bt = bank_transfers::create(&client, &body)
                .await
                .map_err(|e| miette::miette!("{e}"))?;
            let rendered = output::render_single(&bt, global.output, global.compact)
                .map_err(|e| miette::miette!("{e}"))?;
            println!("{rendered}");
        }
    }

    Ok(())
}
