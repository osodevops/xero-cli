use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::bank_transaction::{BankTransaction, BankTransactionsWrapper};
use crate::models::history::HistoryRecord;

#[derive(Default)]
pub struct BankTransactionFilters {
    pub account_id: Option<String>,
    pub date_from: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
}

pub async fn list(
    client: &CachedClient,
    filters: &BankTransactionFilters,
) -> Result<Vec<BankTransaction>> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    let mut where_parts: Vec<String> = Vec::new();

    if let Some(ref account_id) = filters.account_id {
        where_parts.push(format!("BankAccount.AccountID=guid(\"{}\")", account_id));
    }

    if let Some(ref date_from) = filters.date_from {
        where_parts.push(format!("Date>=DateTime({})", date_from));
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

    let response = client.get_with_params("BankTransactions", &params).await?;
    let wrapper: BankTransactionsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.bank_transactions)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<BankTransaction> {
    let response = client.get(&format!("BankTransactions/{id}")).await?;
    let wrapper: BankTransactionsWrapper = serde_json::from_value(response)?;
    wrapper
        .bank_transactions
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Bank transaction not found"))
}

pub async fn create(client: &CachedClient, data: &serde_json::Value) -> Result<BankTransaction> {
    let response = client.put_json("BankTransactions", data).await?;
    let wrapper: BankTransactionsWrapper = serde_json::from_value(response)?;
    wrapper.bank_transactions.into_iter().next().ok_or_else(|| {
        crate::error::XeroCliError::api(500, "No bank transaction returned from create")
    })
}

pub async fn delete(client: &CachedClient, id: &str) -> Result<BankTransaction> {
    let body = serde_json::json!({"Status": "DELETED"});
    let response = client
        .post_json(&format!("BankTransactions/{id}"), &body)
        .await?;
    let wrapper: BankTransactionsWrapper = serde_json::from_value(response)?;
    wrapper.bank_transactions.into_iter().next().ok_or_else(|| {
        crate::error::XeroCliError::api(500, "No bank transaction returned from delete")
    })
}

pub async fn history(client: &CachedClient, id: &str) -> Result<Vec<HistoryRecord>> {
    super::common::get_history(client, "BankTransactions", id).await
}
