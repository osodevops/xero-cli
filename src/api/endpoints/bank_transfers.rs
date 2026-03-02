use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::bank_transfer::{BankTransfer, BankTransfersWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<BankTransfer>> {
    let response = client.get("BankTransfers").await?;
    let wrapper: BankTransfersWrapper = serde_json::from_value(response)?;
    Ok(wrapper.bank_transfers)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<BankTransfer> {
    let response = client.get(&format!("BankTransfers/{id}")).await?;
    let wrapper: BankTransfersWrapper = serde_json::from_value(response)?;
    wrapper
        .bank_transfers
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Bank transfer not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<BankTransfer> {
    let response = client.put_json("BankTransfers", data).await?;
    let wrapper: BankTransfersWrapper = serde_json::from_value(response)?;
    wrapper.bank_transfers.into_iter().next().ok_or_else(|| {
        crate::error::XeroCliError::api(500, "No bank transfer returned from create")
    })
}
