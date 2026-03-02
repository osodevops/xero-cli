use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingTheme {
    #[serde(rename = "BrandingThemeID")]
    pub branding_theme_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "SortOrder")]
    pub sort_order: Option<u32>,
    #[serde(rename = "Type")]
    pub theme_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingThemesWrapper {
    #[serde(rename = "BrandingThemes")]
    pub branding_themes: Vec<BrandingTheme>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_branding_theme() {
        let json = r#"{"BrandingThemeID": "bt-1", "Name": "Standard", "SortOrder": 0}"#;
        let bt: BrandingTheme = serde_json::from_str(json).unwrap();
        assert_eq!(bt.name.as_deref(), Some("Standard"));
    }
    #[test]
    fn deserialize_branding_themes_wrapper() {
        let json = r#"{"BrandingThemes": [{"BrandingThemeID": "bt-1", "Name": "Standard"}]}"#;
        let w: BrandingThemesWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(w.branding_themes.len(), 1);
    }
}
