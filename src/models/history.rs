use serde::{Deserialize, Serialize};

use super::common::deserialize_xero_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    #[serde(rename = "Changes")]
    pub changes: Option<String>,
    #[serde(
        rename = "DateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub date_utc: Option<String>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Details")]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecordsWrapper {
    #[serde(rename = "HistoryRecords")]
    pub history_records: Vec<HistoryRecord>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_history_record() {
        let json = r#"{
            "Changes": "Updated",
            "DateUTC": "2024-01-15T10:30:00",
            "User": "john@example.com",
            "Details": "Invoice approved"
        }"#;
        let record: HistoryRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.changes.as_deref(), Some("Updated"));
        assert_eq!(record.user.as_deref(), Some("john@example.com"));
        assert_eq!(record.details.as_deref(), Some("Invoice approved"));
    }

    #[test]
    fn deserialize_history_records_wrapper() {
        let json = r#"{
            "HistoryRecords": [
                {
                    "Changes": "Created",
                    "DateUTC": "2024-01-10T08:00:00",
                    "User": "admin@example.com",
                    "Details": "Payment created"
                }
            ]
        }"#;
        let wrapper: HistoryRecordsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.history_records.len(), 1);
    }
}
