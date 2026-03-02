use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "TaxType")]
    pub tax_type: Option<String>,
    #[serde(rename = "EffectiveRate")]
    pub effective_rate: Option<Decimal>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "TaxComponents", default)]
    pub tax_components: Vec<TaxComponent>,
    #[serde(rename = "CanApplyToAssets")]
    pub can_apply_to_assets: Option<bool>,
    #[serde(rename = "CanApplyToEquity")]
    pub can_apply_to_equity: Option<bool>,
    #[serde(rename = "CanApplyToExpenses")]
    pub can_apply_to_expenses: Option<bool>,
    #[serde(rename = "CanApplyToLiabilities")]
    pub can_apply_to_liabilities: Option<bool>,
    #[serde(rename = "CanApplyToRevenue")]
    pub can_apply_to_revenue: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxComponent {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Rate")]
    pub rate: Option<Decimal>,
    #[serde(rename = "IsCompound")]
    pub is_compound: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRatesWrapper {
    #[serde(rename = "TaxRates")]
    pub tax_rates: Vec<TaxRate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_tax_rate() {
        let json = r#"{"Name": "Sales Tax", "TaxType": "OUTPUT", "EffectiveRate": 20.0, "Status": "ACTIVE", "TaxComponents": [{"Name": "GST", "Rate": 20.0}]}"#;
        let tr: TaxRate = serde_json::from_str(json).unwrap();
        assert_eq!(tr.name.as_deref(), Some("Sales Tax"));
        assert_eq!(tr.effective_rate, Some(Decimal::new(200, 1)));
    }
    #[test]
    fn deserialize_tax_rates_wrapper() {
        let json = r#"{"TaxRates": [{"Name": "GST", "TaxType": "OUTPUT"}]}"#;
        let w: TaxRatesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.tax_rates.len(), 1);
    }
}
