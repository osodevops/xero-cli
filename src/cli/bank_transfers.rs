use crate::api::endpoints::bank_transfers;
use crate::cli::common::build_client;
use crate::cli::GlobalArgs;
use crate::models::bank_transfer::BankTransfer;
use crate::output::{self, Tabular};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum BankTransferCommands {
    /// List bank transfers
    ///
    /// Retrieves all bank transfers recorded in your Xero organisation.
    /// Each transfer represents a movement of funds between two bank accounts.
    /// Results are returned in reverse chronological order.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transfers list
  xero bank-transfers list --output json
  xero bank-transfers list --compact")]
    List,

    /// Get a specific bank transfer
    ///
    /// Retrieves the full details of a single bank transfer by its unique
    /// Xero identifier, including the source and destination bank accounts,
    /// the transfer amount, and the date.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transfers get 297c2dc5-cc47-4afd-8ec8-74990b8761e9
  xero bank-transfers get 297c2dc5-cc47-4afd-8ec8-74990b8761e9 --output json")]
    Get {
        /// Bank transfer UUID (e.g. 297c2dc5-cc47-4afd-8ec8-74990b8761e9)
        id: String,
    },

    /// Create a bank transfer
    ///
    /// Creates a new bank transfer between two bank accounts in your Xero
    /// organisation. Both account IDs must refer to existing bank accounts,
    /// and the source account must have sufficient funds for the transfer.
    /// The amount must be a positive decimal value.
    #[command(after_long_help = "\
EXAMPLES:
  xero bank-transfers create \\
    --from-account 297c2dc5-cc47-4afd-8ec8-74990b8761e9 \\
    --to-account 5baa2e4c-3c05-4089-a657-c6a76a tried07 \\
    --amount 250.00

  xero bank-transfers create \\
    --from-account <SAVINGS_ACCOUNT_ID> \\
    --to-account <CHEQUE_ACCOUNT_ID> \\
    --amount 1000.00 --output json")]
    Create {
        /// Source bank account UUID to transfer funds from
        #[arg(long)]
        from_account: String,
        /// Destination bank account UUID to transfer funds to
        #[arg(long)]
        to_account: String,
        /// Transfer amount as a positive decimal (e.g. 250.00)
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
