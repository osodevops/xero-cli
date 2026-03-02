use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::budget::{Budget, BudgetsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<Budget>> {
    let response = client.get("Budgets").await?;
    let wrapper: BudgetsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.budgets)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<Budget> {
    let response = client.get(&format!("Budgets/{id}")).await?;
    let wrapper: BudgetsWrapper = serde_json::from_value(response)?;
    wrapper
        .budgets
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Budget not found"))
}
