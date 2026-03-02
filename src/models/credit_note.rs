use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::{deserialize_xero_date, LineItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNote {
    #[serde(rename = "CreditNoteID")]
    pub credit_note_id: Option<String>,
    #[serde(rename = "CreditNoteNumber")]
    pub credit_note_number: Option<String>,
    #[serde(rename = "Type")]
    pub credit_note_type: Option<CreditNoteType>,
    #[serde(rename = "Status")]
    pub status: Option<CreditNoteStatus>,
    #[serde(rename = "Contact")]
    pub contact: Option<CreditNoteContact>,
    #[serde(rename = "LineItems", default)]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "SubTotal")]
    pub sub_total: Option<Decimal>,
    #[serde(rename = "TotalTax")]
    pub total_tax: Option<Decimal>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "RemainingCredit")]
    pub remaining_credit: Option<Decimal>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
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
pub struct CreditNoteContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNotesWrapper {
    #[serde(rename = "CreditNotes")]
    pub credit_notes: Vec<CreditNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CreditNoteType {
    ACCPAYCREDIT,
    ACCRECCREDIT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CreditNoteStatus {
    DRAFT,
    SUBMITTED,
    AUTHORISED,
    PAID,
    VOIDED,
    DELETED,
}

impl std::fmt::Display for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_credit_note() {
        let json = r#"{
            "CreditNoteID": "cn-123",
            "CreditNoteNumber": "CN-001",
            "Type": "ACCRECCREDIT",
            "Status": "AUTHORISED",
            "Contact": {"ContactID": "c-1", "Name": "Customer"},
            "LineItems": [],
            "Total": 200.00,
            "RemainingCredit": 150.00
        }"#;
        let cn: CreditNote = serde_json::from_str(json).unwrap();
        assert_eq!(cn.credit_note_id.as_deref(), Some("cn-123"));
        assert_eq!(cn.credit_note_type, Some(CreditNoteType::ACCRECCREDIT));
        assert_eq!(cn.remaining_credit, Some(Decimal::new(15000, 2)));
    }

    #[test]
    fn deserialize_credit_notes_wrapper() {
        let json = r#"{
            "CreditNotes": [
                {"CreditNoteID": "cn-1", "Status": "DRAFT"}
            ]
        }"#;
        let wrapper: CreditNotesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.credit_notes.len(), 1);
    }

    #[test]
    fn credit_note_status_display() {
        assert_eq!(CreditNoteStatus::AUTHORISED.to_string(), "AUTHORISED");
        assert_eq!(CreditNoteType::ACCPAYCREDIT.to_string(), "ACCPAYCREDIT");
    }
}
