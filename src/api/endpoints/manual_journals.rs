use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::manual_journal::{ManualJournal, ManualJournalsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<ManualJournal>> {
    let response = client.get("ManualJournals").await?;
    let wrapper: ManualJournalsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.manual_journals)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<ManualJournal> {
    let response = client.get(&format!("ManualJournals/{id}")).await?;
    let wrapper: ManualJournalsWrapper = serde_json::from_value(response)?;
    wrapper
        .manual_journals
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Manual journal not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<ManualJournal> {
    let response = client.put_json("ManualJournals", data).await?;
    let wrapper: ManualJournalsWrapper = serde_json::from_value(response)?;
    wrapper
        .manual_journals
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No manual journal returned"))
}

pub async fn update(
    client: &CachedClient,
    id: &str,
    data: &serde_json::Value,
) -> Result<ManualJournal> {
    let response = client
        .post_json(&format!("ManualJournals/{id}"), data)
        .await?;
    let wrapper: ManualJournalsWrapper = serde_json::from_value(response)?;
    wrapper
        .manual_journals
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No manual journal returned"))
}
