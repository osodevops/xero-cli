use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::expense_claim::{ExpenseClaim, ExpenseClaimsWrapper};
use crate::models::history::HistoryRecord;

pub async fn list(client: &CachedClient) -> Result<Vec<ExpenseClaim>> {
    let response = client.get("ExpenseClaims").await?;
    let wrapper: ExpenseClaimsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.expense_claims)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<ExpenseClaim> {
    let response = client.get(&format!("ExpenseClaims/{id}")).await?;
    let wrapper: ExpenseClaimsWrapper = serde_json::from_value(response)?;
    wrapper
        .expense_claims
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Expense claim not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<ExpenseClaim> {
    let response = client.put_json("ExpenseClaims", data).await?;
    let wrapper: ExpenseClaimsWrapper = serde_json::from_value(response)?;
    wrapper
        .expense_claims
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No expense claim returned"))
}

pub async fn update(
    client: &CachedClient,
    id: &str,
    data: &serde_json::Value,
) -> Result<ExpenseClaim> {
    let response = client
        .post_json(&format!("ExpenseClaims/{id}"), data)
        .await?;
    let wrapper: ExpenseClaimsWrapper = serde_json::from_value(response)?;
    wrapper
        .expense_claims
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No expense claim returned"))
}

pub async fn history(client: &CachedClient, id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "ExpenseClaims", id).await
}
