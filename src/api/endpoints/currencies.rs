use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::currency::{CurrenciesWrapper, Currency};

pub async fn list(client: &CachedClient) -> Result<Vec<Currency>> {
    let response = client.get("Currencies").await?;
    let wrapper: CurrenciesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.currencies)
}
