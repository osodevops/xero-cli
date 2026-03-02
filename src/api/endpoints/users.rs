use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::user::{User, UsersWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<User>> {
    let response = client.get("Users").await?;
    let wrapper: UsersWrapper = serde_json::from_value(response)?;
    Ok(wrapper.users)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<User> {
    let response = client.get(&format!("Users/{id}")).await?;
    let wrapper: UsersWrapper = serde_json::from_value(response)?;
    wrapper
        .users
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "User not found"))
}
