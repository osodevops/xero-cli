use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::organisation::{Organisation, OrganisationsWrapper};

pub async fn get(client: &CachedClient) -> Result<Organisation> {
    let response = client.get("Organisation").await?;
    let wrapper: OrganisationsWrapper = serde_json::from_value(response)?;
    wrapper
        .organisations
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Organisation not found"))
}
