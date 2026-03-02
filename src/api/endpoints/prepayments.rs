use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::allocation::Allocation;
use crate::models::history::HistoryRecord;
use crate::models::prepayment::{Prepayment, PrepaymentsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<Prepayment>> {
    let response = client.get("Prepayments").await?;
    let wrapper: PrepaymentsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.prepayments)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<Prepayment> {
    let response = client.get(&format!("Prepayments/{id}")).await?;
    let wrapper: PrepaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .prepayments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Prepayment not found"))
}

pub async fn allocate(
    client: &CachedClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Vec<Allocation>> {
    super::common::create_allocation(client, "Prepayments", id, body).await
}

pub async fn history(client: &CachedClient, id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "Prepayments", id).await
}
