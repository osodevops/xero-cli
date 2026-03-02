use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::quote::{Quote, QuotesWrapper};

#[derive(Default)]
pub struct QuoteFilters {
    pub status: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
}

pub async fn list(client: &CachedClient, filters: &QuoteFilters) -> Result<Vec<Quote>> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    let mut where_parts: Vec<String> = Vec::new();

    if let Some(ref status) = filters.status {
        where_parts.push(format!("Status==\"{}\"", status));
    }

    if let Some(ref where_clause) = filters.where_clause {
        where_parts.push(where_clause.clone());
    }

    let where_val = where_parts.join("&&");
    if !where_val.is_empty() {
        params.push(("where", &where_val));
    }

    let order_val;
    if let Some(ref order) = filters.order {
        order_val = order.clone();
        params.push(("order", &order_val));
    }

    let response = client.get_with_params("Quotes", &params).await?;
    let wrapper: QuotesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.quotes)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<Quote> {
    let response = client.get(&format!("Quotes/{id}")).await?;
    let wrapper: QuotesWrapper = serde_json::from_value(response)?;
    wrapper
        .quotes
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Quote not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<Quote> {
    let response = client.put_json("Quotes", data).await?;
    let wrapper: QuotesWrapper = serde_json::from_value(response)?;
    wrapper
        .quotes
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No quote returned from create"))
}

pub async fn update(client: &CachedClient, id: &str, updates: &serde_json::Value) -> Result<Quote> {
    let response = client.post_json(&format!("Quotes/{id}"), updates).await?;
    let wrapper: QuotesWrapper = serde_json::from_value(response)?;
    wrapper
        .quotes
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No quote returned from update"))
}
