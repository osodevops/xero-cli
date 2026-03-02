use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::payment_service::{PaymentService, PaymentServicesWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<PaymentService>> {
    let response = client.get("PaymentServices").await?;
    let wrapper: PaymentServicesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.payment_services)
}
