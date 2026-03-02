use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::repeating_invoice::{RepeatingInvoice, RepeatingInvoicesWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<RepeatingInvoice>> {
    let response = client.get("RepeatingInvoices").await?;
    let wrapper: RepeatingInvoicesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.repeating_invoices)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<RepeatingInvoice> {
    let response = client.get(&format!("RepeatingInvoices/{id}")).await?;
    let wrapper: RepeatingInvoicesWrapper = serde_json::from_value(response)?;
    wrapper
        .repeating_invoices
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Repeating invoice not found"))
}
