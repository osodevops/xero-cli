use super::common::deserialize_xero_date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedTransaction {
    #[serde(rename = "LinkedTransactionID")]
    pub linked_transaction_id: Option<String>,
    #[serde(rename = "SourceTransactionID")]
    pub source_transaction_id: Option<String>,
    #[serde(rename = "SourceLineItemID")]
    pub source_line_item_id: Option<String>,
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "TargetTransactionID")]
    pub target_transaction_id: Option<String>,
    #[serde(rename = "TargetLineItemID")]
    pub target_line_item_id: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    pub transaction_type: Option<String>,
    #[serde(rename = "SourceTransactionTypeCode")]
    pub source_type_code: Option<String>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedTransactionsWrapper {
    #[serde(rename = "LinkedTransactions")]
    pub linked_transactions: Vec<LinkedTransaction>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_linked_transaction() {
        let json =
            r#"{"LinkedTransactionID": "lt-1", "Status": "APPROVED", "Type": "BILLABLEEXPENSE"}"#;
        let lt: LinkedTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(lt.linked_transaction_id.as_deref(), Some("lt-1"));
    }
    #[test]
    fn deserialize_linked_transactions_wrapper() {
        let json = r#"{"LinkedTransactions": [{"LinkedTransactionID": "lt-1"}]}"#;
        let w: LinkedTransactionsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.linked_transactions.len(), 1);
    }
}
