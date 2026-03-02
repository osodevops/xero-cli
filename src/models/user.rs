use super::common::deserialize_xero_date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserID")]
    pub user_id: Option<String>,
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
    #[serde(rename = "IsSubscriber")]
    pub is_subscriber: Option<bool>,
    #[serde(rename = "OrganisationRole")]
    pub organisation_role: Option<String>,
    #[serde(
        rename = "UpdatedDateUTC",
        deserialize_with = "deserialize_xero_date",
        default
    )]
    pub updated_date_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersWrapper {
    #[serde(rename = "Users")]
    pub users: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_user() {
        let json = r#"{"UserID": "u-1", "EmailAddress": "john@example.com", "FirstName": "John", "LastName": "Doe", "OrganisationRole": "STANDARD"}"#;
        let u: User = serde_json::from_str(json).unwrap();
        assert_eq!(u.email_address.as_deref(), Some("john@example.com"));
    }
    #[test]
    fn deserialize_users_wrapper() {
        let json = r#"{"Users": [{"UserID": "u-1", "FirstName": "John"}]}"#;
        let w: UsersWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.users.len(), 1);
    }
}
