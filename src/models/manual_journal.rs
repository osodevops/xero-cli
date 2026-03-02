use super::common::deserialize_xero_date;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualJournal {
    #[serde(rename = "ManualJournalID")]
    pub manual_journal_id: Option<String>,
    #[serde(rename = "Narration")]
    pub narration: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Date", deserialize_with = "deserialize_xero_date", default)]
    pub date: Option<String>,
    #[serde(rename = "JournalLines", default)]
    pub journal_lines: Vec<JournalLine>,
    #[serde(rename = "Url")]
    pub url: Option<String>,
    #[serde(rename = "ShowOnCashBasisReports")]
    pub show_on_cash_basis_reports: Option<bool>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    #[serde(rename = "LineAmount")]
    pub line_amount: Option<Decimal>,
    #[serde(rename = "AccountCode")]
    pub account_code: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "TaxType")]
    pub tax_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualJournalsWrapper {
    #[serde(rename = "ManualJournals")]
    pub manual_journals: Vec<ManualJournal>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_manual_journal() {
        let json =
            r#"{"ManualJournalID": "mj-1", "Narration": "Test journal", "Status": "POSTED"}"#;
        let mj: ManualJournal = serde_json::from_str(json).unwrap();
        assert_eq!(mj.narration.as_deref(), Some("Test journal"));
    }
    #[test]
    fn deserialize_manual_journals_wrapper() {
        let json = r#"{"ManualJournals": [{"ManualJournalID": "mj-1"}]}"#;
        let w: ManualJournalsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.manual_journals.len(), 1);
    }
}
