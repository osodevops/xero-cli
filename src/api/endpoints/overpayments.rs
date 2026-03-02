use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::allocation::Allocation;
use crate::models::history::HistoryRecord;
use crate::models::overpayment::{Overpayment, OverpaymentsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<Overpayment>> {
    let response = client.get("Overpayments").await?;
    let wrapper: OverpaymentsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.overpayments)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<Overpayment> {
    let response = client.get(&format!("Overpayments/{id}")).await?;
    let wrapper: OverpaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .overpayments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Overpayment not found"))
}

pub async fn allocate(
    client: &CachedClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Vec<Allocation>> {
    super::common::create_allocation(client, "Overpayments", id, body).await
}

pub async fn history(client: &CachedClient, id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "Overpayments", id).await
}
