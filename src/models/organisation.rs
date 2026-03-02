use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organisation {
    #[serde(rename = "OrganisationID")]
    pub organisation_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LegalName")]
    pub legal_name: Option<String>,
    #[serde(rename = "OrganisationType")]
    pub organisation_type: Option<String>,
    #[serde(rename = "CountryCode")]
    pub country_code: Option<String>,
    #[serde(rename = "BaseCurrency")]
    pub base_currency: Option<String>,
    #[serde(rename = "DefaultSalesTax")]
    pub default_sales_tax: Option<String>,
    #[serde(rename = "DefaultPurchasesTax")]
    pub default_purchases_tax: Option<String>,
    #[serde(rename = "FinancialYearEndDay")]
    pub financial_year_end_day: Option<u32>,
    #[serde(rename = "FinancialYearEndMonth")]
    pub financial_year_end_month: Option<u32>,
    #[serde(rename = "ShortCode")]
    pub short_code: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,
    #[serde(rename = "LineOfBusiness")]
    pub line_of_business: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganisationsWrapper {
    #[serde(rename = "Organisations")]
    pub organisations: Vec<Organisation>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_organisation() {
        let json = r#"{"OrganisationID": "o-1", "Name": "My Company", "CountryCode": "GB", "BaseCurrency": "GBP"}"#;
        let o: Organisation = serde_json::from_str(json).unwrap();
        assert_eq!(o.name.as_deref(), Some("My Company"));
    }
    #[test]
    fn deserialize_organisations_wrapper() {
        let json = r#"{"Organisations": [{"OrganisationID": "o-1", "Name": "My Company"}]}"#;
        let w: OrganisationsWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.organisations.len(), 1);
    }
}
