use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactGroup {
    #[serde(rename = "ContactGroupID")]
    pub contact_group_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Contacts", default)]
    pub contacts: Vec<ContactGroupContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactGroupContact {
    #[serde(rename = "ContactID")]
    pub contact_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactGroupsWrapper {
    #[serde(rename = "ContactGroups")]
    pub contact_groups: Vec<ContactGroup>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_contact_group() {
        let json = r#"{"ContactGroupID": "cg-1", "Name": "VIP Customers", "Status": "ACTIVE"}"#;
        let cg: ContactGroup = serde_json::from_str(json).unwrap();
        assert_eq!(cg.name.as_deref(), Some("VIP Customers"));
    }
    #[test]
    fn deserialize_contact_groups_wrapper() {
        let json = r#"{"ContactGroups": [{"ContactGroupID": "cg-1", "Name": "VIP"}]}"#;
        let w: ContactGroupsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.contact_groups.len(), 1);
    }
}
