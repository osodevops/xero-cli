use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::history::HistoryRecord;
use crate::models::receipt::{Receipt, ReceiptsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<Receipt>> {
    let response = client.get("Receipts").await?;
    let wrapper: ReceiptsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.receipts)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<Receipt> {
    let response = client.get(&format!("Receipts/{id}")).await?;
    let wrapper: ReceiptsWrapper = serde_json::from_value(response)?;
    wrapper
        .receipts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Receipt not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<Receipt> {
    let response = client.put_json("Receipts", data).await?;
    let wrapper: ReceiptsWrapper = serde_json::from_value(response)?;
    wrapper
        .receipts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No receipt returned"))
}

pub async fn history(client: &CachedClient, id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "Receipts", id).await
}
