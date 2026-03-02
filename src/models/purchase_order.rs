use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{deserialize_xero_date, LineItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder {
    #[serde(rename = "PurchaseOrderID")]
    pub purchase_order_id: Option<String>,
    #[serde(rename = "PurchaseOrderNumber")]
    pub purchase_order_number: Option<String>,
    #[serde(rename = "Contact")]
    pub contact: Option<PurchaseOrderContact>,
    #[serde(rename = "LineItems", default)]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "Status")]
    pub status: Option<PurchaseOrderStatus>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(
        rename = "DeliveryDate",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub delivery_date: Option<String>,
    #[serde(rename = "SubTotal")]
    pub sub_total: Option<Decimal>,
    #[serde(rename = "TotalTax")]
    pub total_tax: Option<Decimal>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
    #[serde(rename = "CurrencyCode")]
    pub currency_code: Option<String>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrdersWrapper {
    #[serde(rename = "PurchaseOrders")]
    pub purchase_orders: Vec<PurchaseOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PurchaseOrderStatus {
    DRAFT,
    SUBMITTED,
    AUTHORISED,
    BILLED,
    DELETED,
}

impl std::fmt::Display for PurchaseOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_purchase_order() {
        let json = r#"{
            "PurchaseOrderID": "po-123",
            "PurchaseOrderNumber": "PO-001",
            "Contact": {"ContactID": "c-1", "Name": "Supplier"},
            "LineItems": [],
            "Status": "AUTHORISED",
            "Total": 750.00,
            "Date": "2024-01-15"
        }"#;
        let po: PurchaseOrder = serde_json::from_str(json).unwrap();
        assert_eq!(po.purchase_order_id.as_deref(), Some("po-123"));
        assert_eq!(po.status, Some(PurchaseOrderStatus::AUTHORISED));
        assert_eq!(po.total, Some(Decimal::new(75000, 2)));
    }

    #[test]
    fn deserialize_purchase_orders_wrapper() {
        let json = r#"{
            "PurchaseOrders": [
                {"PurchaseOrderID": "po-1", "Status": "DRAFT"}
            ]
        }"#;
        let wrapper: PurchaseOrdersWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.purchase_orders.len(), 1);
    }

    #[test]
    fn purchase_order_status_display() {
        assert_eq!(PurchaseOrderStatus::AUTHORISED.to_string(), "AUTHORISED");
        assert_eq!(PurchaseOrderStatus::BILLED.to_string(), "BILLED");
    }
}
