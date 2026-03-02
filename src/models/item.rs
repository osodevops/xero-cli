use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ItemID")]
    pub item_id: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "PurchaseDescription")]
    pub purchase_description: Option<String>,
    #[serde(rename = "PurchaseDetails")]
    pub purchase_details: Option<ItemPurchaseDetails>,
    #[serde(rename = "SalesDetails")]
    pub sales_details: Option<ItemSaleDetails>,
    #[serde(rename = "IsTrackedAsInventory")]
    pub is_tracked_as_inventory: Option<bool>,
    #[serde(rename = "IsSold")]
    pub is_sold: Option<bool>,
    #[serde(rename = "IsPurchased")]
    pub is_purchased: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPurchaseDetails {
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<Decimal>,
    #[serde(rename = "AccountCode")]
    pub account_code: Option<String>,
    #[serde(rename = "TaxType")]
    pub tax_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSaleDetails {
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<Decimal>,
    #[serde(rename = "AccountCode")]
    pub account_code: Option<String>,
    #[serde(rename = "TaxType")]
    pub tax_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsWrapper {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_item() {
        let json = r#"{
            "ItemID": "item-123",
            "Code": "WDG",
            "Name": "Widget",
            "Description": "A standard widget",
            "SalesDetails": {
                "UnitPrice": 29.99,
                "AccountCode": "200",
                "TaxType": "OUTPUT"
            },
            "PurchaseDetails": {
                "UnitPrice": 15.00,
                "AccountCode": "300",
                "TaxType": "INPUT"
            },
            "IsSold": true,
            "IsPurchased": true
        }"#;
        let item: Item = serde_json::from_str(json).unwrap();
        assert_eq!(item.code.as_deref(), Some("WDG"));
        assert_eq!(item.name.as_deref(), Some("Widget"));
        assert_eq!(
            item.sales_details.as_ref().unwrap().unit_price,
            Some(Decimal::new(2999, 2))
        );
    }

    #[test]
    fn deserialize_items_wrapper() {
        let json = r#"{
            "Items": [
                {
                    "ItemID": "i-1",
                    "Code": "TEST",
                    "Name": "Test Item"
                }
            ]
        }"#;
        let wrapper: ItemsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.items.len(), 1);
    }
}
