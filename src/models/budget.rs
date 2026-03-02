use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    #[serde(rename = "BudgetID")]
    pub budget_id: Option<String>,
    #[serde(rename = "Type")]
    pub budget_type: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "BudgetLines", default)]
    pub budget_lines: Vec<BudgetLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetLine {
    #[serde(rename = "AccountID")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountCode")]
    pub account_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetsWrapper {
    #[serde(rename = "Budgets")]
    pub budgets: Vec<Budget>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_budget() {
        let json = r#"{"BudgetID": "b-1", "Type": "OVERALL", "Description": "Annual Budget"}"#;
        let b: Budget = serde_json::from_str(json).unwrap();
        assert_eq!(b.budget_id.as_deref(), Some("b-1"));
    }
    #[test]
    fn deserialize_budgets_wrapper() {
        let json = r#"{"Budgets": [{"BudgetID": "b-1", "Type": "OVERALL"}]}"#;
        let w: BudgetsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.budgets.len(), 1);
    }
}
