use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::contact::{Contact, ContactsWrapper};

#[derive(Default)]
pub struct ContactFilters {
    pub search: Option<String>,
    pub where_clause: Option<String>,
    pub order: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub modified_since: Option<String>,
}

pub async fn list(client: &CachedClient, filters: &ContactFilters) -> Result<Vec<Contact>> {
    let mut params: Vec<(&str, &str)> = Vec::new();

    let search_val;
    if let Some(ref search) = filters.search {
        search_val = search.clone();
        params.push(("searchTerm", &search_val));
    }

    let where_val;
    if let Some(ref where_clause) = filters.where_clause {
        where_val = where_clause.clone();
        params.push(("where", &where_val));
    }

    let order_val;
    if let Some(ref order) = filters.order {
        order_val = order.clone();
        params.push(("order", &order_val));
    }

    let page_val;
    if let Some(page) = filters.page {
        page_val = page.to_string();
        params.push(("page", &page_val));
    }

    let page_size_val;
    if let Some(page_size) = filters.page_size {
        page_size_val = page_size.to_string();
        params.push(("pageSize", &page_size_val));
    }

    let response = client.get_with_params("Contacts", &params).await?;
    let wrapper: ContactsWrapper = serde_json::from_value(response)?;
    Ok(wrapper.contacts)
}

pub async fn get(client: &CachedClient, contact_id: &str) -> Result<Contact> {
    let response = client.get(&format!("Contacts/{contact_id}")).await?;
    let wrapper: ContactsWrapper = serde_json::from_value(response)?;
    wrapper
        .contacts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Contact not found"))
}

pub async fn create(client: &CachedClient, contact: &serde_json::Value) -> Result<Contact> {
    let response = client.put_json("Contacts", contact).await?;
    let wrapper: ContactsWrapper = serde_json::from_value(response)?;
    wrapper
        .contacts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No contact returned from create"))
}

pub async fn update(
    client: &CachedClient,
    contact_id: &str,
    updates: &serde_json::Value,
) -> Result<Contact> {
    let response = client
        .post_json(&format!("Contacts/{contact_id}"), updates)
        .await?;
    let wrapper: ContactsWrapper = serde_json::from_value(response)?;
    wrapper
        .contacts
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(500, "No contact returned from update"))
}
