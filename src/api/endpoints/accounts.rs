use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::account::{Account, AccountsWrapper};

#[derive(Default)]
pub struct AccountFilters {
    pub account_type: Option<String>,
    pub class: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
}

pub async fn list(client: &CachedClient, filters: &AccountFilters) -> Result<Vec<Account>> {
    let mut params: Vec<(&str, &str)> = Vec::new();

    let mut where_parts: Vec<String> = Vec::new();

    if let Some(ref account_type) = filters.account_type {
        where_parts.push(format!("Type==\"{}\"", account_type));
    }

    if let Some(ref class) = filters.class {
        where_parts.push(format!("Class==\"{}\"", class));
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

    let response = client.get_with_params("Accounts", &params).await?;
    let wrapper: AccountsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.accounts)
}

pub async fn get(client: &CachedClient, account_id: &str) -> Result<Account> {
    let response = client.get(&format!("Accounts/{account_id}")).await?;
    let wrapper: AccountsWrapper = serde_json::from_value(response)?;
    wrapper
        .accounts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Account not found"))
}

pub async fn create(client: &CachedClient, account: &serde_json::Value) -> Result<Account> {
    let response = client.put_json("Accounts", account).await?;
    let wrapper: AccountsWrapper = serde_json::from_value(response)?;
    wrapper
        .accounts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No account returned from create"))
}

pub async fn archive(client: &CachedClient, account_id: &str) -> Result<Account> {
    // Xero archives accounts via DELETE
    let response = client.delete(&format!("Accounts/{account_id}")).await?;
    let wrapper: AccountsWrapper = serde_json::from_value(response)?;
    wrapper
        .accounts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No account returned from archive"))
}
