use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::journal::{Journal, JournalsWrapper};

pub async fn list(client: &CachedClient, offset: Option<u64>) -> Result<Vec<Journal>> {
    let offset_str = offset.unwrap_or(0).to_string();
    let params = vec![("offset", offset_str.as_str())];
    let response = client.get_with_params("Journals", &params).await?;
    let wrapper: JournalsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.journals)
}

pub async fn list_all(client: &CachedClient) -> Result<Vec<Journal>> {
    crate::api::pagination::paginate_all_offset(client, "Journals", &[], 100, |v| {
        serde_json::from_value::<JournalsWrapper>(v.clone())
            .ok()
            .map(|w| w.journals)
    })
    .await
}

pub async fn get(client: &CachedClient, id: &str) -> Result<Journal> {
    let response = client.get(&format!("Journals/{id}")).await?;
    let wrapper: JournalsWrapper = serde_json::from_value(response)?;
    wrapper
        .journals
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Journal not found"))
}
