use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::history::HistoryRecord;
use crate::models::item::{Item, ItemsWrapper};

#[derive(Default)]
pub struct ItemFilters {
    pub where_clause: Option<String>,
    pub order: Option<String>,
}

pub async fn list(client: &CachedClient, filters: &ItemFilters) -> Result<Vec<Item>> {
    let mut params: Vec<(&str, &str)> = Vec::new();

    let where_val;
    if let Some(ref where_clause) = filters.where_clause {
        where_val = where_clause.clone();
        params.push(("where", &where_val));
    }

    let order_val;
    if let Some(ref order) = filters.order {
        order_val = order.clone();
        params.push(("order", &order_val));
    }

    let response = client.get_with_params("Items", &params).await?;
    let wrapper: ItemsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.items)
}

pub async fn get(client: &CachedClient, item_id: &str) -> Result<Item> {
    let response = client.get(&format!("Items/{item_id}")).await?;
    let wrapper: ItemsWrapper = serde_json::from_value(response)?;
    wrapper
        .items
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Item not found"))
}

pub async fn create(client: &CachedClient, item: &serde_json::Value) -> Result<Item> {
    let response = client.put_json("Items", item).await?;
    let wrapper: ItemsWrapper = serde_json::from_value(response)?;
    wrapper
        .items
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No item returned from create"))
}

pub async fn update(
    client: &CachedClient,
    item_id: &str,
    updates: &serde_json::Value,
) -> Result<Item> {
    let response = client
        .post_json(&format!("Items/{item_id}"), updates)
        .await?;
    let wrapper: ItemsWrapper = serde_json::from_value(response)?;
    wrapper
        .items
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No item returned from update"))
}

pub async fn delete(client: &CachedClient, item_id: &str) -> Result<()> {
    client.delete(&format!("Items/{item_id}")).await?;
    Ok(())
}

pub async fn history(client: &CachedClient, item_id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "Items", item_id).await
}
