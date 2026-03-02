use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "AccountID")]
    pub account_id: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub account_type: Option<AccountType>,
    #[serde(rename = "Class")]
    pub class: Option<AccountClass>,
    #[serde(rename = "Status")]
    pub status: Option<AccountStatus>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "TaxType")]
    pub tax_type: Option<String>,
    #[serde(rename = "EnablePaymentsToAccount")]
    pub enable_payments: Option<bool>,
    #[serde(rename = "ShowInExpenseClaims")]
    pub show_in_expense_claims: Option<bool>,
    #[serde(rename = "BankAccountNumber")]
    pub bank_account_number: Option<String>,
    #[serde(rename = "CurrencyCode")]
    pub currency_code: Option<String>,
    #[serde(rename = "ReportingCode")]
    pub reporting_code: Option<String>,
    #[serde(rename = "HasAttachments")]
    pub has_attachments: Option<bool>,
    #[serde(rename = "SystemAccount")]
    pub system_account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsWrapper {
    #[serde(rename = "Accounts")]
    pub accounts: Vec<Account>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    BANK,
    CURRENT,
    CURRLIAB,
    DEPRECIATN,
    DIRECTCOSTS,
    EQUITY,
    EXPENSE,
    FIXED,
    INVENTORY,
    LIABILITY,
    NONCURRENT,
    OTHERINCOME,
    OVERHEADS,
    PREPAYMENT,
    REVENUE,
    SALES,
    TERMLIAB,
    PAYGLIABILITY,
    SUPERANNUATIONEXPENSE,
    SUPERANNUATIONLIABILITY,
    WAGESEXPENSE,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountClass {
    ASSET,
    EQUITY,
    EXPENSE,
    LIABILITY,
    REVENUE,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountStatus {
    ACTIVE,
    ARCHIVED,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for AccountClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_account() {
        let json = r#"{
            "AccountID": "acc-123",
            "Code": "200",
            "Name": "Sales",
            "Type": "REVENUE",
            "Class": "REVENUE",
            "Status": "ACTIVE",
            "TaxType": "OUTPUT"
        }"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.name.as_deref(), Some("Sales"));
        assert_eq!(account.account_type, Some(AccountType::REVENUE));
        assert_eq!(account.class, Some(AccountClass::REVENUE));
    }

    #[test]
    fn deserialize_accounts_wrapper() {
        let json = r#"{
            "Accounts": [
                {"AccountID": "a-1", "Name": "Bank", "Type": "BANK", "Status": "ACTIVE"}
            ]
        }"#;
        let wrapper: AccountsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.accounts.len(), 1);
    }
}
