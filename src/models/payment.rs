use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::deserialize_xero_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "PaymentID")]
    pub payment_id: Option<String>,
    #[serde(rename = "Invoice")]
    pub invoice: Option<PaymentInvoice>,
    #[serde(rename = "Account")]
    pub account: Option<PaymentAccount>,
    #[serde(rename = "Amount")]
    pub amount: Option<Decimal>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<PaymentStatus>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
    #[serde(rename = "CurrencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "PaymentType")]
    pub payment_type: Option<String>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInvoice {
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Option<String>,
    #[serde(rename = "InvoiceNumber")]
    pub invoice_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAccount {
    #[serde(rename = "AccountID")]
    pub account_id: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentsWrapper {
    #[serde(rename = "Payments")]
    pub payments: Vec<Payment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    AUTHORISED,
    DELETED,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_payment() {
        let json = r#"{
            "PaymentID": "pay-123",
            "Invoice": {
                "InvoiceID": "inv-456",
                "InvoiceNumber": "INV-001"
            },
            "Account": {
                "AccountID": "acc-789",
                "Code": "090"
            },
            "Amount": 500.00,
            "Date": "2024-01-15",
            "Status": "AUTHORISED",
            "Reference": "Payment for INV-001"
        }"#;
        let payment: Payment = serde_json::from_str(json).unwrap();
        assert_eq!(payment.payment_id.as_deref(), Some("pay-123"));
        assert_eq!(payment.amount, Some(Decimal::new(50000, 2)));
        assert_eq!(payment.status, Some(PaymentStatus::AUTHORISED));
    }

    #[test]
    fn deserialize_payments_wrapper() {
        let json = r#"{
            "Payments": [
                {
                    "PaymentID": "pay-1",
                    "Amount": 100.00,
                    "Status": "AUTHORISED"
                }
            ]
        }"#;
        let wrapper: PaymentsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.payments.len(), 1);
    }

    #[test]
    fn payment_status_display() {
        assert_eq!(PaymentStatus::AUTHORISED.to_string(), "AUTHORISED");
        assert_eq!(PaymentStatus::DELETED.to_string(), "DELETED");
    }
}
