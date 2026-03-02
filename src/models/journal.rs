use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::common::deserialize_xero_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    #[serde(rename = "JournalID")]
    pub journal_id: Option<String>,
    #[serde(rename = "JournalNumber")]
    pub journal_number: Option<i64>,
    #[serde(
        rename = "JournalDate",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub journal_date: Option<String>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
    #[serde(rename = "SourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "SourceType")]
    pub source_type: Option<String>,
    #[serde(rename = "JournalLines")]
    pub journal_lines: Option<Vec<JournalLine>>,
    #[serde(
        rename = "CreatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub created_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    #[serde(rename = "JournalLineID")]
    pub journal_line_id: Option<String>,
    #[serde(rename = "AccountID")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountCode")]
    pub account_code: Option<String>,
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "NetAmount")]
    pub net_amount: Option<Decimal>,
    #[serde(rename = "GrossAmount")]
    pub gross_amount: Option<Decimal>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: Option<Decimal>,
    #[serde(rename = "TaxType")]
    pub tax_type: Option<String>,
    #[serde(rename = "TaxName")]
    pub tax_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalsWrapper {
    #[serde(rename = "Journals")]
    pub journals: Vec<Journal>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_journal() {
        let json = r#"{
            "JournalID": "j-123",
            "JournalNumber": 42,
            "JournalDate": "2024-01-15",
            "SourceType": "ACCREC",
            "JournalLines": [
                {
                    "JournalLineID": "jl-1",
                    "AccountCode": "200",
                    "AccountName": "Sales",
                    "NetAmount": 500.00,
                    "GrossAmount": 575.00,
                    "TaxAmount": 75.00
                }
            ]
        }"#;
        let j: Journal = serde_json::from_str(json).unwrap();
        assert_eq!(j.journal_id.as_deref(), Some("j-123"));
        assert_eq!(j.journal_number, Some(42));
        assert_eq!(j.journal_lines.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn deserialize_journals_wrapper() {
        let json = r#"{
            "Journals": [
                {
                    "JournalID": "j-1",
                    "JournalNumber": 1,
                    "SourceType": "ACCREC"
                }
            ]
        }"#;
        let wrapper: JournalsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.journals.len(), 1);
    }

    #[test]
    fn deserialize_journal_line() {
        let json = r#"{
            "JournalLineID": "jl-1",
            "AccountCode": "200",
            "AccountName": "Sales",
            "NetAmount": 100.00,
            "GrossAmount": 115.00,
            "TaxAmount": 15.00,
            "TaxType": "OUTPUT",
            "TaxName": "GST on Income"
        }"#;
        let jl: JournalLine = serde_json::from_str(json).unwrap();
        assert_eq!(jl.account_code.as_deref(), Some("200"));
        assert_eq!(jl.net_amount, Some(Decimal::new(10000, 2)));
    }
}
