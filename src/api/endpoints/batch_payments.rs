use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::batch_payment::{BatchPayment, BatchPaymentsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<BatchPayment>> {
    let response = client.get("BatchPayments").await?;
    let wrapper: BatchPaymentsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.batch_payments)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<BatchPayment> {
    let response = client.get(&format!("BatchPayments/{id}")).await?;
    let wrapper: BatchPaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .batch_payments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Batch payment not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<BatchPayment> {
    let response = client.put_json("BatchPayments", data).await?;
    let wrapper: BatchPaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .batch_payments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No batch payment returned"))
}

pub async fn delete(client: &CachedClient, id: &str) -> Result<BatchPayment> {
    let body = serde_json::json!({"Status": "DELETED"});
    let response = client
        .post_json(&format!("BatchPayments/{id}"), &body)
        .await?;
    let wrapper: BatchPaymentsWrapper = serde_json::from_value(response)?;
    wrapper
        .batch_payments
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No batch payment returned"))
}
