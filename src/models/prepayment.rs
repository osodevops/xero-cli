use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::deserialize_xero_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prepayment {
    #[serde(rename = "PrepaymentID")]
    pub prepayment_id: Option<String>,
    #[serde(rename = "Type")]
    pub prepayment_type: Option<String>,
    #[serde(rename = "Contact")]
    pub contact: Option<PrepaymentContact>,
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
pub struct PrepaymentContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepaymentsWrapper {
    #[serde(rename = "Prepayments")]
    pub prepayments: Vec<Prepayment>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_prepayment() {
        let json = r#"{
            "PrepaymentID": "pp-123",
            "Type": "RECEIVE-PREPAYMENT",
            "Contact": {"ContactID": "c-1", "Name": "Test Co"},
            "Status": "AUTHORISED",
            "Total": 1000.00,
            "RemainingCredit": 750.00,
            "Date": "2024-06-01"
        }"#;
        let pp: Prepayment = serde_json::from_str(json).unwrap();
        assert_eq!(pp.prepayment_id.as_deref(), Some("pp-123"));
        assert_eq!(pp.total, Some(Decimal::new(100000, 2)));
        assert_eq!(pp.remaining_credit, Some(Decimal::new(75000, 2)));
    }

    #[test]
    fn deserialize_prepayments_wrapper() {
        let json = r#"{
            "Prepayments": [
                {
                    "PrepaymentID": "pp-1",
                    "Total": 500.00,
                    "Status": "AUTHORISED"
                }
            ]
        }"#;
        let wrapper: PrepaymentsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.prepayments.len(), 1);
    }
}
