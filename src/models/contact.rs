use serde::{Deserialize, Serialize};

use super::common::{Address, Phone};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "ContactStatus")]
    pub contact_status: Option<ContactStatus>,
    #[serde(rename = "AccountNumber")]
    pub account_number: Option<String>,
    #[serde(rename = "TaxNumber")]
    pub tax_number: Option<String>,
    #[serde(rename = "IsSupplier")]
    pub is_supplier: Option<bool>,
    #[serde(rename = "IsCustomer")]
    pub is_customer: Option<bool>,
    #[serde(rename = "DefaultCurrency")]
    pub default_currency: Option<String>,
    #[serde(rename = "Addresses", default)]
    pub addresses: Vec<Address>,
    #[serde(rename = "Phones", default)]
    pub phones: Vec<Phone>,
    #[serde(rename = "ContactPersons", default)]
    pub contact_persons: Vec<ContactPerson>,
    #[serde(rename = "HasAttachments")]
    pub has_attachments: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPerson {
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "IncludeInEmails")]
    pub include_in_emails: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactsWrapper {
    #[serde(rename = "Contacts")]
    pub contacts: Vec<Contact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContactStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "ARCHIVED")]
    Archived,
    #[serde(rename = "GDPRREQUEST")]
    GdprRequest,
}

impl std::fmt::Display for ContactStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "ACTIVE"),
            Self::Archived => write!(f, "ARCHIVED"),
            Self::GdprRequest => write!(f, "GDPRREQUEST"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_contact() {
        let json = r#"{
            "ContactID": "abc-123",
            "Name": "Acme Corp",
            "EmailAddress": "info@acme.com",
            "ContactStatus": "ACTIVE",
            "IsCustomer": true,
            "IsSupplier": false,
            "Addresses": [],
            "Phones": []
        }"#;
        let contact: Contact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.name.as_deref(), Some("Acme Corp"));
        assert_eq!(contact.contact_status, Some(ContactStatus::Active));
        assert!(contact.is_customer.unwrap());
    }

    #[test]
    fn deserialize_contacts_wrapper() {
        let json = r#"{
            "Contacts": [
                {"ContactID": "c-1", "Name": "Company A"},
                {"ContactID": "c-2", "Name": "Company B"}
            ]
        }"#;
        let wrapper: ContactsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.contacts.len(), 2);
    }
}
