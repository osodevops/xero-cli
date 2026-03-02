use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::employee::{Employee, EmployeesWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<Employee>> {
    let response = client.get("Employees").await?;
    let wrapper: EmployeesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.employees)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<Employee> {
    let response = client.get(&format!("Employees/{id}")).await?;
    let wrapper: EmployeesWrapper = serde_json::from_value(response)?;
    wrapper
        .employees
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Employee not found"))
}
