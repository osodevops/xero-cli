use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::tracking_category::{
    TrackingCategoriesWrapper, TrackingCategory, TrackingOption, TrackingOptionsWrapper,
};

pub async fn list(client: &CachedClient) -> Result<Vec<TrackingCategory>> {
    let response = client.get("TrackingCategories").await?;
    let wrapper: TrackingCategoriesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.tracking_categories)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<TrackingCategory> {
    let response = client.get(&format!("TrackingCategories/{id}")).await?;
    let wrapper: TrackingCategoriesWrapper = serde_json::from_value(response)?;
    wrapper
        .tracking_categories
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Tracking category not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<TrackingCategory> {
    let response = client.put_json("TrackingCategories", data).await?;
    let wrapper: TrackingCategoriesWrapper = serde_json::from_value(response)?;
    wrapper
        .tracking_categories
        .into_iter()
        .next()
        .ok_or_else(|| {
            crate::error::XeroCliError::api(500, "No tracking category returned from create")
        })
}

pub async fn update(
    client: &CachedClient,
    id: &str,
    data: &serde_json::Value,
) -> Result<TrackingCategory> {
    let response = client
        .post_json(&format!("TrackingCategories/{id}"), data)
        .await?;
    let wrapper: TrackingCategoriesWrapper = serde_json::from_value(response)?;
    wrapper
        .tracking_categories
        .into_iter()
        .next()
        .ok_or_else(|| {
            crate::error::XeroCliError::api(500, "No tracking category returned from update")
        })
}

pub async fn add_option(
    client: &CachedClient,
    category_id: &str,
    data: &serde_json::Value,
) -> Result<TrackingOption> {
    let response = client
        .put_json(&format!("TrackingCategories/{category_id}/Options"), data)
        .await?;
    let wrapper: TrackingOptionsWrapper = serde_json::from_value(response)?;
    wrapper
        .options
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No option returned from create"))
}

pub async fn update_option(
    client: &CachedClient,
    category_id: &str,
    option_id: &str,
    data: &serde_json::Value,
) -> Result<TrackingOption> {
    let response = client
        .post_json(
            &format!("TrackingCategories/{category_id}/Options/{option_id}"),
            data,
        )
        .await?;
    let wrapper: TrackingOptionsWrapper = serde_json::from_value(response)?;
    wrapper
        .options
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No option returned from update"))
}

pub async fn delete_option(
    client: &CachedClient,
    category_id: &str,
    option_id: &str,
) -> Result<()> {
    client
        .delete(&format!(
            "TrackingCategories/{category_id}/Options/{option_id}"
        ))
        .await?;
    Ok(())
}
