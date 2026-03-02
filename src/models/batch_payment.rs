use super::common::deserialize_xero_date;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPayment {
    #[serde(rename = "BatchPaymentID")]
    pub batch_payment_id: Option<String>,
    #[serde(rename = "Account")]
    pub account: Option<BatchPaymentAccount>,
    #[serde(rename = "Payments", default)]
    pub payments: Vec<serde_json::Value>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "TotalAmount")]
    pub total_amount: Option<Decimal>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
    #[serde(rename = "Type")]
    pub batch_type: Option<String>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPaymentAccount {
    #[serde(rename = "AccountID")]
    pub account_id: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPaymentsWrapper {
    #[serde(rename = "BatchPayments")]
    pub batch_payments: Vec<BatchPayment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_batch_payment() {
        let json = r#"{"BatchPaymentID": "bp-1", "Status": "AUTHORISED", "TotalAmount": 5000.00}"#;
        let bp: BatchPayment = serde_json::from_str(json).unwrap();
        assert_eq!(bp.batch_payment_id.as_deref(), Some("bp-1"));
    }
    #[test]
    fn deserialize_batch_payments_wrapper() {
        let json = r#"{"BatchPayments": [{"BatchPaymentID": "bp-1"}]}"#;
        let w: BatchPaymentsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.batch_payments.len(), 1);
    }
}
