use super::common::deserialize_xero_date;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseClaim {
    #[serde(rename = "ExpenseClaimID")]
    pub expense_claim_id: Option<String>,
    #[serde(rename = "User")]
    pub user: Option<ExpenseClaimUser>,
    #[serde(rename = "Receipts", default)]
    pub receipts: Vec<serde_json::Value>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Total")]
    pub total: Option<Decimal>,
    #[serde(rename = "AmountDue")]
    pub amount_due: Option<Decimal>,
    #[serde(rename = "AmountPaid")]
    pub amount_paid: Option<Decimal>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseClaimUser {
    #[serde(rename = "UserID")]
    pub user_id: Option<String>,
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseClaimsWrapper {
    #[serde(rename = "ExpenseClaims")]
    pub expense_claims: Vec<ExpenseClaim>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_expense_claim() {
        let json = r#"{"ExpenseClaimID": "ec-1", "Status": "SUBMITTED", "Total": 250.00}"#;
        let ec: ExpenseClaim = serde_json::from_str(json).unwrap();
        assert_eq!(ec.expense_claim_id.as_deref(), Some("ec-1"));
    }
    #[test]
    fn deserialize_expense_claims_wrapper() {
        let json = r#"{"ExpenseClaims": [{"ExpenseClaimID": "ec-1"}]}"#;
        let w: ExpenseClaimsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.expense_claims.len(), 1);
    }
}
