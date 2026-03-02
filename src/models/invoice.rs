use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{deserialize_xero_date, LineItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Option<String>,
    #[serde(rename = "InvoiceNumber")]
    pub invoice_number: Option<String>,
    #[serde(rename = "Type")]
    pub invoice_type: Option<InvoiceType>,
    #[serde(rename = "Status")]
    pub status: Option<InvoiceStatus>,
    #[serde(rename = "Contact")]
    pub contact: Option<InvoiceContact>,
    #[serde(rename = "LineItems", default)]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(
        rename = "DueDate",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub due_date: Option<String>,
    #[serde(rename = "SubTotal")]
    pub sub_total: Option<Decimal>,
    #[serde(rename = "TotalTax")]
    pub total_tax: Option<Decimal>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "AmountDue")]
    pub amount_due: Option<Decimal>,
    #[serde(rename = "AmountPaid")]
    pub amount_paid: Option<Decimal>,
    #[serde(rename = "AmountCredited")]
    pub amount_credited: Option<Decimal>,
    #[serde(rename = "CurrencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,
    #[serde(rename = "HasAttachments")]
    pub has_attachments: Option<bool>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicesWrapper {
    #[serde(rename = "Invoices")]
    pub invoices: Vec<Invoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvoiceType {
    #[serde(rename = "ACCPAY")]
    AccountsPayable,
    #[serde(rename = "ACCREC")]
    AccountsReceivable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvoiceStatus {
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "SUBMITTED")]
    Submitted,
    #[serde(rename = "AUTHORISED")]
    Authorised,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "VOIDED")]
    Voided,
    #[serde(rename = "DELETED")]
    Deleted,
}

impl std::fmt::Display for InvoiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountsPayable => write!(f, "ACCPAY"),
            Self::AccountsReceivable => write!(f, "ACCREC"),
        }
    }
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "DRAFT"),
            Self::Submitted => write!(f, "SUBMITTED"),
            Self::Authorised => write!(f, "AUTHORISED"),
            Self::Paid => write!(f, "PAID"),
            Self::Voided => write!(f, "VOIDED"),
            Self::Deleted => write!(f, "DELETED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_invoice() {
        let json = r#"{
            "InvoiceID": "abc-123",
            "InvoiceNumber": "INV-001",
            "Type": "ACCREC",
            "Status": "AUTHORISED",
            "Contact": {
                "ContactID": "contact-123",
                "Name": "Acme Corp"
            },
            "LineItems": [],
            "SubTotal": 1000.00,
            "TotalTax": 150.00,
            "Total": 1150.00,
            "AmountDue": 1150.00,
            "CurrencyCode": "GBP"
        }"#;
        let invoice: Invoice = serde_json::from_str(json).unwrap();
        assert_eq!(invoice.invoice_id.as_deref(), Some("abc-123"));
        assert_eq!(invoice.invoice_type, Some(InvoiceType::AccountsReceivable));
        assert_eq!(invoice.status, Some(InvoiceStatus::Authorised));
        assert_eq!(invoice.total, Some(Decimal::new(115000, 2)));
    }

    #[test]
    fn deserialize_invoices_wrapper() {
        let json = r#"{
            "Invoices": [
                {
                    "InvoiceID": "inv-1",
                    "InvoiceNumber": "INV-001",
                    "Status": "DRAFT"
                }
            ]
        }"#;
        let wrapper: InvoicesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.invoices.len(), 1);
    }

    #[test]
    fn invoice_status_display() {
        assert_eq!(InvoiceStatus::Authorised.to_string(), "AUTHORISED");
        assert_eq!(InvoiceType::AccountsPayable.to_string(), "ACCPAY");
    }
}
