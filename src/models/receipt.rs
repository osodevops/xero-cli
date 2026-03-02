use super::common::{deserialize_xero_date, LineItem};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    #[serde(rename = "ReceiptID")]
    pub receipt_id: Option<String>,
    #[serde(rename = "ReceiptNumber")]
    pub receipt_number: Option<String>,
    #[serde(rename = "Contact")]
    pub contact: Option<ReceiptContact>,
    #[serde(rename = "User")]
    pub user: Option<ReceiptUser>,
    #[serde(rename = "LineItems", default)]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
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
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptUser {
    #[serde(rename = "UserID")]
    pub user_id: Option<String>,
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptsWrapper {
    #[serde(rename = "Receipts")]
    pub receipts: Vec<Receipt>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_receipt() {
        let json = r#"{"ReceiptID": "r-1", "ReceiptNumber": "REC-001", "Status": "DRAFT", "Total": 100.00}"#;
        let r: Receipt = serde_json::from_str(json).unwrap();
        assert_eq!(r.receipt_id.as_deref(), Some("r-1"));
    }
    #[test]
    fn deserialize_receipts_wrapper() {
        let json = r#"{"Receipts": [{"ReceiptID": "r-1"}]}"#;
        let w: ReceiptsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.receipts.len(), 1);
    }
}
