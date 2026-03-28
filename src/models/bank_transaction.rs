use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{deserialize_xero_date, LineItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    #[serde(rename = "BankTransactionID")]
    pub bank_transaction_id: Option<String>,
    #[serde(rename = "Type")]
    pub transaction_type: Option<BankTransactionType>,
    #[serde(rename = "Contact")]
    pub contact: Option<BankTransactionContact>,
    #[serde(rename = "BankAccount")]
    pub bank_account: Option<BankTransactionAccount>,
    #[serde(rename = "LineItems", default)]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "Status")]
    pub status: Option<BankTransactionStatus>,
    #[serde(rename = "SubTotal")]
    pub sub_total: Option<Decimal>,
    #[serde(rename = "TotalTax")]
    pub total_tax: Option<Decimal>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
    #[serde(rename = "CurrencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "IsReconciled")]
    pub is_reconciled: Option<bool>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransactionContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransactionAccount {
    #[serde(rename = "AccountID")]
    pub account_id: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransactionsWrapper {
    #[serde(rename = "BankTransactions")]
    pub bank_transactions: Vec<BankTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankTransactionType {
    SPEND,
    RECEIVE,
    #[serde(rename = "SPEND-OVERPAYMENT")]
    SpendOverpayment,
    #[serde(rename = "RECEIVE-OVERPAYMENT")]
    ReceiveOverpayment,
    #[serde(rename = "SPEND-PREPAYMENT")]
    SpendPrepayment,
    #[serde(rename = "RECEIVE-PREPAYMENT")]
    ReceivePrepayment,
    #[serde(rename = "SPEND-TRANSFER")]
    SpendTransfer,
    #[serde(rename = "RECEIVE-TRANSFER")]
    ReceiveTransfer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankTransactionStatus {
    AUTHORISED,
    DELETED,
}

impl std::fmt::Display for BankTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SPEND => write!(f, "SPEND"),
            Self::RECEIVE => write!(f, "RECEIVE"),
            Self::SpendOverpayment => write!(f, "SPEND-OVERPAYMENT"),
            Self::ReceiveOverpayment => write!(f, "RECEIVE-OVERPAYMENT"),
            Self::SpendPrepayment => write!(f, "SPEND-PREPAYMENT"),
            Self::ReceivePrepayment => write!(f, "RECEIVE-PREPAYMENT"),
            Self::SpendTransfer => write!(f, "SPEND-TRANSFER"),
            Self::ReceiveTransfer => write!(f, "RECEIVE-TRANSFER"),
        }
    }
}

impl std::fmt::Display for BankTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_bank_transaction() {
        let json = r#"{
            "BankTransactionID": "bt-123",
            "Type": "SPEND",
            "Contact": {"ContactID": "c-1", "Name": "Supplier"},
            "BankAccount": {"AccountID": "a-1", "Code": "090"},
            "LineItems": [],
            "Status": "AUTHORISED",
            "Total": 250.00,
            "Date": "2024-01-15"
        }"#;
        let bt: BankTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(bt.bank_transaction_id.as_deref(), Some("bt-123"));
        assert_eq!(bt.transaction_type, Some(BankTransactionType::SPEND));
        assert_eq!(bt.total, Some(Decimal::new(25000, 2)));
    }

    #[test]
    fn deserialize_bank_transactions_wrapper() {
        let json = r#"{
            "BankTransactions": [
                {"BankTransactionID": "bt-1", "Type": "RECEIVE", "Status": "AUTHORISED"}
            ]
        }"#;
        let wrapper: BankTransactionsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.bank_transactions.len(), 1);
    }

    #[test]
    fn bank_transaction_type_display() {
        assert_eq!(BankTransactionType::SPEND.to_string(), "SPEND");
        assert_eq!(BankTransactionType::RECEIVE.to_string(), "RECEIVE");
        assert_eq!(
            BankTransactionType::SpendTransfer.to_string(),
            "SPEND-TRANSFER"
        );
        assert_eq!(
            BankTransactionType::ReceiveTransfer.to_string(),
            "RECEIVE-TRANSFER"
        );
    }

    #[test]
    fn deserialize_spend_transfer() {
        let json = r#"{
            "BankTransactionID": "bt-456",
            "Type": "SPEND-TRANSFER",
            "Status": "AUTHORISED"
        }"#;
        let bt: BankTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(
            bt.transaction_type,
            Some(BankTransactionType::SpendTransfer)
        );
    }

    #[test]
    fn deserialize_receive_transfer() {
        let json = r#"{
            "BankTransactionID": "bt-789",
            "Type": "RECEIVE-TRANSFER",
            "Status": "AUTHORISED"
        }"#;
        let bt: BankTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(
            bt.transaction_type,
            Some(BankTransactionType::ReceiveTransfer)
        );
    }
}
