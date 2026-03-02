use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::history::HistoryRecord;
use crate::models::payment::{Payment, PaymentsWrapper};

#[derive(Default)]
pub struct PaymentFilters {
    pub invoice_id: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
    pub modified_since: Option<String>,
}

pub async fn list(client: &CachedClient, filters: &PaymentFilters) -> Result<Vec<Payment>> {
    let mut params: Vec<(&str, &str)> = Vec::new();

    let where_val;
    if let Some(ref invoice_id) = filters.invoice_id {
        where_val = format!("Invoice.InvoiceID=guid(\"{}\")", invoice_id);
        params.push(("where", &where_val));
    } else if let Some(ref where_clause) = filters.where_clause {
        where_val = where_clause.clone();
        params.push(("where", &where_val));
    }

    let order_val;
    if let Some(ref order) = filters.order {
        order_val = order.clone();
        params.push(("order", &order_val));
    }

    let response = client.get_with_params("Payments", &params).await?;
    let wrapper: PaymentsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.payments)
}

pub async fn get(client: &CachedClient, payment_id: &str) -> Result<Payment> {
    let response = client.get(&format!("Payments/{payment_id}")).await?;
    let wrapper: PaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .payments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Payment not found"))
}

pub async fn create(client: &CachedClient, payment: &serde_json::Value) -> Result<Payment> {
    let response = client.put_json("Payments", payment).await?;
    let wrapper: PaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .payments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No payment returned from create"))
}

pub async fn delete(client: &CachedClient, payment_id: &str) -> Result<Payment> {
    let body = serde_json::json!({"Status": "DELETED"});
    let response = client
        .post_json(&format!("Payments/{payment_id}"), &body)
        .await?;
    let wrapper: PaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .payments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No payment returned from delete"))
}

pub async fn history(client: &CachedClient, payment_id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "Payments", payment_id).await
}
