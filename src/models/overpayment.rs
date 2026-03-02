use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::deserialize_xero_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overpayment {
    #[serde(rename = "OverpaymentID")]
    pub overpayment_id: Option<String>,
    #[serde(rename = "Type")]
    pub overpayment_type: Option<String>,
    #[serde(rename = "Contact")]
    pub contact: Option<OverpaymentContact>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "SubTotal")]
    pub sub_total: Option<Decimal>,
    #[serde(rename = "TotalTax")]
    pub total_tax: Option<Decimal>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "RemainingCredit")]
    pub remaining_credit: Option<Decimal>,
    #[serde(rename = "CurrencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "LineItems")]
    pub line_items: Option<Vec<super::common::LineItem>>,
    #[serde(rename = "Allocations")]
    pub allocations: Option<Vec<super::allocation::Allocation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverpaymentContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverpaymentsWrapper {
    #[serde(rename = "Overpayments")]
    pub overpayments: Vec<Overpayment>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_overpayment() {
        let json = r#"{
            "OverpaymentID": "op-123",
            "Type": "RECEIVE-OVERPAYMENT",
            "Contact": {"ContactID": "c-1", "Name": "Test Co"},
            "Status": "AUTHORISED",
            "Total": 500.00,
            "RemainingCredit": 200.00,
            "Date": "2024-03-15"
        }"#;
        let op: Overpayment = serde_json::from_str(json).unwrap();
        assert_eq!(op.overpayment_id.as_deref(), Some("op-123"));
        assert_eq!(op.total, Some(Decimal::new(50000, 2)));
        assert_eq!(op.remaining_credit, Some(Decimal::new(20000, 2)));
    }

    #[test]
    fn deserialize_overpayments_wrapper() {
        let json = r#"{
            "Overpayments": [
                {
                    "OverpaymentID": "op-1",
                    "Total": 100.00,
                    "Status": "AUTHORISED"
                }
            ]
        }"#;
        let wrapper: OverpaymentsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.overpayments.len(), 1);
    }
}
