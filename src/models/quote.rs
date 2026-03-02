use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{deserialize_xero_date, LineItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "QuoteID")]
    pub quote_id: Option<String>,
    #[serde(rename = "QuoteNumber")]
    pub quote_number: Option<String>,
    #[serde(rename = "Contact")]
    pub contact: Option<QuoteContact>,
    #[serde(rename = "LineItems", default)]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "Status")]
    pub status: Option<QuoteStatus>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(
        rename = "ExpiryDate",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub expiry_date: Option<String>,
    #[serde(rename = "SubTotal")]
    pub sub_total: Option<Decimal>,
    #[serde(rename = "TotalTax")]
    pub total_tax: Option<Decimal>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Summary")]
    pub summary: Option<String>,
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
pub struct QuoteContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesWrapper {
    #[serde(rename = "Quotes")]
    pub quotes: Vec<Quote>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuoteStatus {
    DRAFT,
    SENT,
    ACCEPTED,
    DECLINED,
    INVOICED,
    DELETED,
}

impl std::fmt::Display for QuoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_quote() {
        let json = r#"{
            "QuoteID": "q-123",
            "QuoteNumber": "QU-001",
            "Contact": {"ContactID": "c-1", "Name": "Customer"},
            "LineItems": [],
            "Status": "DRAFT",
            "Total": 1500.00,
            "Date": "2024-01-15",
            "ExpiryDate": "2024-02-15"
        }"#;
        let quote: Quote = serde_json::from_str(json).unwrap();
        assert_eq!(quote.quote_id.as_deref(), Some("q-123"));
        assert_eq!(quote.status, Some(QuoteStatus::DRAFT));
        assert_eq!(quote.total, Some(Decimal::new(150000, 2)));
    }

    #[test]
    fn deserialize_quotes_wrapper() {
        let json = r#"{
            "Quotes": [
                {"QuoteID": "q-1", "Status": "SENT"}
            ]
        }"#;
        let wrapper: QuotesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.quotes.len(), 1);
    }

    #[test]
    fn quote_status_display() {
        assert_eq!(QuoteStatus::DRAFT.to_string(), "DRAFT");
        assert_eq!(QuoteStatus::INVOICED.to_string(), "INVOICED");
    }
}
