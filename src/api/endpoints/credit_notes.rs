use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::allocation::Allocation;
use crate::models::credit_note::{CreditNote, CreditNotesWrapper};
use crate::models::history::HistoryRecord;

#[derive(Default)]
pub struct CreditNoteFilters {
    pub status: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
}

pub async fn list(client: &CachedClient, filters: &CreditNoteFilters) -> Result<Vec<CreditNote>> {
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

    let response = client.get_with_params("CreditNotes", &params).await?;
    let wrapper: CreditNotesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.credit_notes)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<CreditNote> {
    let response = client.get(&format!("CreditNotes/{id}")).await?;
    let wrapper: CreditNotesWrapper = serde_json::from_value(response)?;
    wrapper
        .credit_notes
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Credit note not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<CreditNote> {
    let response = client.put_json("CreditNotes", data).await?;
    let wrapper: CreditNotesWrapper = serde_json::from_value(response)?;
    wrapper
        .credit_notes
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No credit note returned from create"))
}

pub async fn allocate(
    client: &CachedClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Vec<Allocation>> {
    super::common::create_allocation(client, "CreditNotes", id, body).await
}

pub async fn history(client: &CachedClient, id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "CreditNotes", id).await
}
