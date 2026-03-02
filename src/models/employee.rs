use super::common::deserialize_xero_date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    #[serde(rename = "EmployeeID")]
    pub employee_id: Option<String>,
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "ExternalLink")]
    pub external_link: Option<serde_json::Value>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeesWrapper {
    #[serde(rename = "Employees")]
    pub employees: Vec<Employee>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_employee() {
        let json =
            r#"{"EmployeeID": "e-1", "FirstName": "John", "LastName": "Doe", "Status": "ACTIVE"}"#;
        let e: Employee = serde_json::from_str(json).unwrap();
        assert_eq!(e.first_name.as_deref(), Some("John"));
    }
    #[test]
    fn deserialize_employees_wrapper() {
        let json = r#"{"Employees": [{"EmployeeID": "e-1", "FirstName": "John"}]}"#;
        let w: EmployeesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.employees.len(), 1);
    }
}
