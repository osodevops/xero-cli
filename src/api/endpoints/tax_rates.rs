use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::tax_rate::{TaxRate, TaxRatesWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<TaxRate>> {
    let response = client.get("TaxRates").await?;
    let wrapper: TaxRatesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.tax_rates)
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<TaxRate> {
    let response = client.put_json("TaxRates", data).await?;
    let wrapper: TaxRatesWrapper = serde_json::from_value(response)?;
    wrapper
        .tax_rates
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No tax rate returned from create"))
}

pub async fn update(client: &CachedClient, data: &serde_json::Value) -> Result<TaxRate> {
    let response = client.post_json("TaxRates", data).await?;
    let wrapper: TaxRatesWrapper = serde_json::from_value(response)?;
    wrapper
        .tax_rates
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No tax rate returned from update"))
}
