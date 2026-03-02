use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XeroResponse<T> {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "AddressType")]
    pub address_type: Option<String>,
    #[serde(rename = "AddressLine1")]
    pub address_line1: Option<String>,
    #[serde(rename = "AddressLine2")]
    pub address_line2: Option<String>,
    #[serde(rename = "City")]
    pub city: Option<String>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "PostalCode")]
    pub postal_code: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phone {
    #[serde(rename = "PhoneType")]
    pub phone_type: Option<String>,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: Option<String>,
    #[serde(rename = "PhoneAreaCode")]
    pub phone_area_code: Option<String>,
    #[serde(rename = "PhoneCountryCode")]
    pub phone_country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    #[serde(rename = "LineItemID")]
    pub line_item_id: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(
        rename = "Quantity",
        deserialize_with = "deserialize_decimal_opt",
        default
    )]
    pub quantity: Option<Decimal>,
    #[serde(
        rename = "UnitAmount",
        deserialize_with = "deserialize_decimal_opt",
        default
    )]
    pub unit_amount: Option<Decimal>,
    #[serde(
        rename = "LineAmount",
        deserialize_with = "deserialize_decimal_opt",
        default
    )]
    pub line_amount: Option<Decimal>,
    #[serde(
        rename = "TaxAmount",
        deserialize_with = "deserialize_decimal_opt",
        default
    )]
    pub tax_amount: Option<Decimal>,
    #[serde(rename = "AccountCode")]
    pub account_code: Option<String>,
    #[serde(rename = "TaxType")]
    pub tax_type: Option<String>,
}

/// Deserialize Xero's date format: `/Date(1234567890000+0000)/`
pub fn deserialize_xero_date<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref raw) if raw.starts_with("/Date(") => {
            let inner = raw.trim_start_matches("/Date(").trim_end_matches(")/");
            // Handle timezone offset: /Date(1234567890000+0000)/
            let ms_str = inner.split('+').next().unwrap_or(inner);
            let ms_str = ms_str.split('-').next().unwrap_or(ms_str);
            if let Ok(ms) = ms_str.parse::<i64>() {
                let dt =
                    chrono::DateTime::from_timestamp(ms / 1000, ((ms % 1000) * 1_000_000) as u32);
                Ok(dt.map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()))
            } else {
                Ok(Some(raw.clone()))
            }
        }
        other => Ok(other),
    }
}

fn deserialize_decimal_opt<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum DecimalOrString {
        Decimal(Decimal),
        String(String),
    }

    let val: Option<DecimalOrString> = Option::deserialize(deserializer)?;
    match val {
        Some(DecimalOrString::Decimal(d)) => Ok(Some(d)),
        Some(DecimalOrString::String(s)) => s
            .parse::<Decimal>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xero_date() {
        let json = r#""/Date(1609459200000+0000)/""#;
        let raw: String = serde_json::from_str(json).unwrap();
        assert!(raw.starts_with("/Date("));
    }

    #[test]
    fn deserialize_line_item() {
        let json = r#"{
            "LineItemID": "abc-123",
            "Description": "Consulting",
            "Quantity": 2.0,
            "UnitAmount": 150.00,
            "LineAmount": 300.00,
            "TaxAmount": 0.00,
            "AccountCode": "200",
            "TaxType": "NONE"
        }"#;
        let item: LineItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.description.as_deref(), Some("Consulting"));
        assert_eq!(item.quantity, Some(Decimal::new(20, 1)));
    }

    #[test]
    fn deserialize_address() {
        let json = r#"{
            "AddressType": "POBOX",
            "City": "London",
            "Country": "UK"
        }"#;
        let addr: Address = serde_json::from_str(json).unwrap();
        assert_eq!(addr.city.as_deref(), Some("London"));
    }
}
