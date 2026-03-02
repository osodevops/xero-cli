use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::deserialize_xero_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransfer {
    #[serde(rename = "BankTransferID")]
    pub bank_transfer_id: Option<String>,
    #[serde(rename = "FromBankAccount")]
    pub from_bank_account: Option<BankTransferAccount>,
    #[serde(rename = "ToBankAccount")]
    pub to_bank_account: Option<BankTransferAccount>,
    #[serde(rename = "Amount")]
    pub amount: Option<Decimal>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(rename = "CurrencyRate")]
    pub currency_rate: Option<Decimal>,
    #[serde(rename = "FromBankTransactionID")]
    pub from_bank_transaction_id: Option<String>,
    #[serde(rename = "ToBankTransactionID")]
    pub to_bank_transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferAccount {
    #[serde(rename = "AccountID")]
    pub account_id: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransfersWrapper {
    #[serde(rename = "BankTransfers")]
    pub bank_transfers: Vec<BankTransfer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_bank_transfer() {
        let json = r#"{
            "BankTransferID": "xfr-123",
            "FromBankAccount": {"AccountID": "a-1", "Name": "Business Account"},
            "ToBankAccount": {"AccountID": "a-2", "Name": "Savings"},
            "Amount": 1000.00,
            "Date": "2024-01-15"
        }"#;
        let bt: BankTransfer = serde_json::from_str(json).unwrap();
        assert_eq!(bt.bank_transfer_id.as_deref(), Some("xfr-123"));
        assert_eq!(bt.amount, Some(Decimal::new(100000, 2)));
    }

    #[test]
    fn deserialize_bank_transfers_wrapper() {
        let json = r#"{
            "BankTransfers": [
                {"BankTransferID": "xfr-1", "Amount": 500.00}
            ]
        }"#;
        let wrapper: BankTransfersWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.bank_transfers.len(), 1);
    }
}
