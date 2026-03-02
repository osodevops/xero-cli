use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::deserialize_xero_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    #[serde(rename = "AllocationID")]
    pub allocation_id: Option<String>,
    #[serde(rename = "Invoice")]
    pub invoice: Option<AllocationInvoice>,
    #[serde(rename = "Amount")]
    pub amount: Option<Decimal>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInvoice {
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Option<String>,
    #[serde(rename = "InvoiceNumber")]
    pub invoice_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationsWrapper {
    #[serde(rename = "Allocations")]
    pub allocations: Vec<Allocation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_allocation() {
        let json = r#"{
            "AllocationID": "alloc-123",
            "Invoice": {
                "InvoiceID": "inv-456",
                "InvoiceNumber": "INV-001"
            },
            "Amount": 50.00,
            "Date": "2024-01-15"
        }"#;
        let alloc: Allocation = serde_json::from_str(json).unwrap();
        assert_eq!(alloc.allocation_id.as_deref(), Some("alloc-123"));
        assert_eq!(alloc.amount, Some(Decimal::new(5000, 2)));
        assert_eq!(
            alloc.invoice.as_ref().unwrap().invoice_id.as_deref(),
            Some("inv-456")
        );
    }

    #[test]
    fn deserialize_allocations_wrapper() {
        let json = r#"{
            "Allocations": [
                {
                    "AllocationID": "alloc-1",
                    "Amount": 100.00
                }
            ]
        }"#;
        let wrapper: AllocationsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.allocations.len(), 1);
    }
}
