use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::linked_transaction::{LinkedTransaction, LinkedTransactionsWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<LinkedTransaction>> {
    let response = client.get("LinkedTransactions").await?;
    let wrapper: LinkedTransactionsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.linked_transactions)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<LinkedTransaction> {
    let response = client.get(&format!("LinkedTransactions/{id}")).await?;
    let wrapper: LinkedTransactionsWrapper = serde_json::from_value(response)?;
    wrapper
        .linked_transactions
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Linked transaction not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<LinkedTransaction> {
    let response = client.put_json("LinkedTransactions", data).await?;
    let wrapper: LinkedTransactionsWrapper = serde_json::from_value(response)?;
    wrapper
        .linked_transactions
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No linked transaction returned"))
}

pub async fn update(
    client: &CachedClient,
    id: &str,
    data: &serde_json::Value,
) -> Result<LinkedTransaction> {
    let response = client
        .post_json(&format!("LinkedTransactions/{id}"), data)
        .await?;
    let wrapper: LinkedTransactionsWrapper = serde_json::from_value(response)?;
    wrapper
        .linked_transactions
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No linked transaction returned"))
}

pub async fn delete(client: &CachedClient, id: &str) -> Result<()> {
    client.delete(&format!("LinkedTransactions/{id}")).await?;
    Ok(())
}
