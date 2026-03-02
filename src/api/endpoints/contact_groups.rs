use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::contact_group::{ContactGroup, ContactGroupsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<ContactGroup>> {
    let response = client.get("ContactGroups").await?;
    let wrapper: ContactGroupsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.contact_groups)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<ContactGroup> {
    let response = client.get(&format!("ContactGroups/{id}")).await?;
    let wrapper: ContactGroupsWrapper = serde_json::from_value(response)?;
    wrapper
        .contact_groups
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Contact group not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<ContactGroup> {
    let response = client.put_json("ContactGroups", data).await?;
    let wrapper: ContactGroupsWrapper = serde_json::from_value(response)?;
    wrapper
        .contact_groups
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No contact group returned"))
}

pub async fn update(
    client: &CachedClient,
    id: &str,
    data: &serde_json::Value,
) -> Result<ContactGroup> {
    let response = client
        .post_json(&format!("ContactGroups/{id}"), data)
        .await?;
    let wrapper: ContactGroupsWrapper = serde_json::from_value(response)?;
    wrapper
        .contact_groups
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No contact group returned"))
}

pub async fn delete(client: &CachedClient, id: &str) -> Result<()> {
    let body = serde_json::json!({"Status": "DELETED"});
    client
        .post_json(&format!("ContactGroups/{id}"), &body)
        .await?;
    Ok(())
}
