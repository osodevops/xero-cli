use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "ReportID")]
    pub report_id: Option<String>,
    #[serde(rename = "ReportName")]
    pub report_name: Option<String>,
    #[serde(rename = "ReportType")]
    pub report_type: Option<String>,
    #[serde(rename = "ReportDate")]
    pub report_date: Option<String>,
    #[serde(rename = "UpdatedDateUTC")]
    pub updated_date_utc: Option<String>,
    #[serde(rename = "Rows", default)]
    pub rows: Vec<ReportRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRow {
    #[serde(rename = "RowType")]
    pub row_type: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Cells", default)]
    pub cells: Vec<ReportCell>,
    #[serde(rename = "Rows", default)]
    pub rows: Vec<ReportRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCell {
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsWrapper {
    #[serde(rename = "Reports")]
    pub reports: Vec<Report>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_report() {
        let json = r#"{
            "ReportID": "ProfitAndLoss",
            "ReportName": "Profit and Loss",
            "ReportType": "ProfitAndLoss",
            "Rows": [
                {
                    "RowType": "Header",
                    "Cells": [
                        {"Value": "Account"},
                        {"Value": "Amount"}
                    ]
                },
                {
                    "RowType": "Section",
                    "Title": "Income",
                    "Rows": [
                        {
                            "RowType": "Row",
                            "Cells": [
                                {"Value": "Sales"},
                                {"Value": "10000.00"}
                            ]
                        }
                    ]
                }
            ]
        }"#;
        let report: Report = serde_json::from_str(json).unwrap();
        assert_eq!(report.report_name.as_deref(), Some("Profit and Loss"));
        assert_eq!(report.rows.len(), 2);
        assert_eq!(report.rows[1].rows.len(), 1);
    }

    #[test]
    fn deserialize_reports_wrapper() {
        let json = r#"{
            "Reports": [
                {
                    "ReportID": "BalanceSheet",
                    "ReportName": "Balance Sheet",
                    "Rows": []
                }
            ]
        }"#;
        let wrapper: ReportsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.reports.len(), 1);
    }
}
