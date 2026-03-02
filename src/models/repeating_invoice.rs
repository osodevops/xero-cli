use super::common::LineItem;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatingInvoice {
    #[serde(rename = "RepeatingInvoiceID")]
    pub repeating_invoice_id: Option<String>,
    #[serde(rename = "Type")]
    pub invoice_type: Option<String>,
    #[serde(rename = "Contact")]
    pub contact: Option<RepeatingInvoiceContact>,
    #[serde(rename = "Schedule")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "LineItems", default)]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "SubTotal")]
    pub sub_total: Option<Decimal>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatingInvoiceContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "Period")]
    pub period: Option<u32>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "DueDate")]
    pub due_date: Option<u32>,
    #[serde(rename = "DueDateType")]
    pub due_date_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatingInvoicesWrapper {
    #[serde(rename = "RepeatingInvoices")]
    pub repeating_invoices: Vec<RepeatingInvoice>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_repeating_invoice() {
        let json = r#"{"RepeatingInvoiceID": "ri-1", "Type": "ACCREC", "Status": "AUTHORISED", "Total": 500.00}"#;
        let ri: RepeatingInvoice = serde_json::from_str(json).unwrap();
        assert_eq!(ri.repeating_invoice_id.as_deref(), Some("ri-1"));
    }
    #[test]
    fn deserialize_repeating_invoices_wrapper() {
        let json =
            r#"{"RepeatingInvoices": [{"RepeatingInvoiceID": "ri-1", "Status": "AUTHORISED"}]}"#;
        let w: RepeatingInvoicesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.repeating_invoices.len(), 1);
    }
}
