use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub tenant_id: String,
    #[serde(default)]
    pub org_name: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

pub const SCOPE_PRESET_READ_ONLY: &[&str] = &[
    "openid",
    "offline_access",
    "accounting.invoices.read",
    "accounting.payments.read",
    "accounting.banktransactions.read",
    "accounting.manualjournals.read",
    "accounting.contacts.read",
    "accounting.settings.read",
    "accounting.reports.profitandloss.read",
    "accounting.reports.balancesheet.read",
    "accounting.reports.aged.read",
    "accounting.reports.banksummary.read",
    "accounting.reports.trialbalance.read",
    "accounting.attachments.read",
];

pub const SCOPE_PRESET_BOOKKEEPER: &[&str] = &[
    "openid",
    "offline_access",
    "accounting.invoices.read",
    "accounting.invoices",
    "accounting.payments.read",
    "accounting.payments",
    "accounting.banktransactions.read",
    "accounting.banktransactions",
    "accounting.manualjournals.read",
    "accounting.manualjournals",
    "accounting.contacts.read",
    "accounting.contacts",
    "accounting.settings.read",
    "accounting.reports.profitandloss.read",
    "accounting.reports.balancesheet.read",
    "accounting.reports.aged.read",
    "accounting.attachments.read",
    "accounting.attachments",
];

pub const SCOPE_PRESET_FULL_ACCESS: &[&str] = &[
    "openid",
    "offline_access",
    "accounting.invoices",
    "accounting.invoices.read",
    "accounting.payments",
    "accounting.payments.read",
    "accounting.banktransactions",
    "accounting.banktransactions.read",
    "accounting.manualjournals",
    "accounting.manualjournals.read",
    "accounting.classicexpenses",
    "accounting.classicexpenses.read",
    "accounting.contacts",
    "accounting.contacts.read",
    "accounting.settings",
    "accounting.settings.read",
    "accounting.reports.profitandloss.read",
    "accounting.reports.balancesheet.read",
    "accounting.reports.aged.read",
    "accounting.reports.banksummary.read",
    "accounting.reports.trialbalance.read",
    "accounting.reports.executivesummary.read",
    "accounting.reports.taxreports.read",
    "accounting.attachments",
    "accounting.attachments.read",
    "accounting.budgets.read",
];

pub const SCOPE_PRESET_REPORTS_ONLY: &[&str] = &[
    "openid",
    "offline_access",
    "accounting.reports.profitandloss.read",
    "accounting.reports.balancesheet.read",
    "accounting.reports.aged.read",
    "accounting.reports.banksummary.read",
    "accounting.reports.trialbalance.read",
    "accounting.reports.executivesummary.read",
    "accounting.reports.taxreports.read",
    "accounting.settings.read",
];

pub fn scope_preset(name: &str) -> Option<Vec<String>> {
    match name {
        "read-only" => Some(
            SCOPE_PRESET_READ_ONLY
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ),
        "bookkeeper" => Some(
            SCOPE_PRESET_BOOKKEEPER
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ),
        "full-access" => Some(
            SCOPE_PRESET_FULL_ACCESS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ),
        "reports-only" => Some(
            SCOPE_PRESET_REPORTS_ONLY
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_presets_exist() {
        assert!(scope_preset("read-only").is_some());
        assert!(scope_preset("bookkeeper").is_some());
        assert!(scope_preset("full-access").is_some());
        assert!(scope_preset("reports-only").is_some());
        assert!(scope_preset("nonexistent").is_none());
    }

    #[test]
    fn read_only_has_offline_access() {
        let scopes = scope_preset("read-only").unwrap();
        assert!(scopes.contains(&"offline_access".to_string()));
    }

    #[test]
    fn bookkeeper_includes_write_scopes() {
        let scopes = scope_preset("bookkeeper").unwrap();
        assert!(scopes.contains(&"accounting.invoices".to_string()));
        assert!(scopes.contains(&"accounting.contacts".to_string()));
    }

    #[test]
    fn profile_deserialization() {
        let toml_str = r#"
tenant_id = "abc-123"
org_name = "Test Org"
scopes = ["openid", "offline_access"]
"#;
        let profile: Profile = toml::from_str(toml_str).unwrap();
        assert_eq!(profile.tenant_id, "abc-123");
        assert_eq!(profile.org_name.as_deref(), Some("Test Org"));
        assert_eq!(profile.scopes.len(), 2);
    }
}
