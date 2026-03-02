use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::allocation::{Allocation, AllocationsWrapper};
use crate::models::history::{HistoryRecord, HistoryRecordsWrapper};

pub async fn get_history(
    client: &CachedClient,
    resource_path: &str,
    id: &str,
) -> Result<Vec<HistoryRecord>> {
    let response = client.get(&format!("{resource_path}/{id}/History")).await?;
    let wrapper: HistoryRecordsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.history_records)
}

pub async fn create_allocation(
    client: &CachedClient,
    resource_path: &str,
    id: &str,
    body: &serde_json::Value,
) -> Result<Vec<Allocation>> {
    let response = client
        .put_json(&format!("{resource_path}/{id}/Allocations"), body)
        .await?;
    let wrapper: AllocationsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.allocations)
}
