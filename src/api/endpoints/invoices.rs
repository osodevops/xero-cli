use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::invoice::{Invoice, InvoicesWrapper};

#[derive(Default)]
pub struct InvoiceFilters {
    pub status: Option<String>,
    pub contact_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub modified_since: Option<String>,
}

pub async fn list(client: &CachedClient, filters: &InvoiceFilters) -> Result<Vec<Invoice>> {
    let mut params: Vec<(&str, &str)> = Vec::new();

    let status_val;
    if let Some(ref status) = filters.status {
        params.push(("Statuses", {
            status_val = status.clone();
            &status_val
        }));
    }

    let contact_val;
    if let Some(ref contact_id) = filters.contact_id {
        contact_val = format!("Contact.ContactID=guid(\"{}\")", contact_id);
        params.push(("where", &contact_val));
    }

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

    let page_val;
    if let Some(page) = filters.page {
        page_val = page.to_string();
        params.push(("page", &page_val));
    }

    let page_size_val;
    if let Some(page_size) = filters.page_size {
        page_size_val = page_size.to_string();
        params.push(("pageSize", &page_size_val));
    }

    let response = client.get_with_params("Invoices", &params).await?;
    let wrapper: InvoicesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.invoices)
}

pub async fn get(client: &CachedClient, invoice_id: &str) -> Result<Invoice> {
    let response = client.get(&format!("Invoices/{invoice_id}")).await?;
    let wrapper: InvoicesWrapper = serde_json::from_value(response)?;
    wrapper
        .invoices
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Invoice not found"))
}

pub async fn create(client: &CachedClient, invoice: &serde_json::Value) -> Result<Invoice> {
    // Xero uses PUT for creates
    let response = client.put_json("Invoices", invoice).await?;
    let wrapper: InvoicesWrapper = serde_json::from_value(response)?;
    wrapper
        .invoices
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No invoice returned from create"))
}

pub async fn update(
    client: &CachedClient,
    invoice_id: &str,
    updates: &serde_json::Value,
) -> Result<Invoice> {
    // Xero uses POST for updates
    let response = client
        .post_json(&format!("Invoices/{invoice_id}"), updates)
        .await?;
    let wrapper: InvoicesWrapper = serde_json::from_value(response)?;
    wrapper
        .invoices
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No invoice returned from update"))
}
