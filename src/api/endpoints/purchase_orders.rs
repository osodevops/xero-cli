use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::history::HistoryRecord;
use crate::models::purchase_order::{PurchaseOrder, PurchaseOrdersWrapper};

#[derive(Default)]
pub struct PurchaseOrderFilters {
    pub status: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
}

pub async fn list(
    client: &CachedClient,
    filters: &PurchaseOrderFilters,
) -> Result<Vec<PurchaseOrder>> {
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

    let response = client.get_with_params("PurchaseOrders", &params).await?;
    let wrapper: PurchaseOrdersWrapper = serde_json::from_value(response)?;
    Ok(wrapper.purchase_orders)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<PurchaseOrder> {
    let response = client.get(&format!("PurchaseOrders/{id}")).await?;
    let wrapper: PurchaseOrdersWrapper = serde_json::from_value(response)?;
    wrapper
        .purchase_orders
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Purchase order not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<PurchaseOrder> {
    let response = client.put_json("PurchaseOrders", data).await?;
    let wrapper: PurchaseOrdersWrapper = serde_json::from_value(response)?;
    wrapper.purchase_orders.into_iter().next().ok_or_else(|| {
        crate::error::XeroCliError::api(500, "No purchase order returned from create")
    })
}

pub async fn history(client: &CachedClient, id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "PurchaseOrders", id).await
}
