use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrenciesWrapper {
    #[serde(rename = "Currencies")]
    pub currencies: Vec<Currency>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_currency() {
        let json = r#"{"Code": "USD", "Description": "United States Dollar"}"#;
        let c: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(c.code.as_deref(), Some("USD"));
    }

    #[test]
    fn deserialize_currencies_wrapper() {
        let json = r#"{"Currencies": [{"Code": "GBP", "Description": "British Pound"}]}"#;
        let w: CurrenciesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.currencies.len(), 1);
    }
}
