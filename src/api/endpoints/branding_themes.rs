use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::branding_theme::{BrandingTheme, BrandingThemesWrapper};

pub async fn list(client: &CachedClient) -> Result<Vec<BrandingTheme>> {
    let response = client.get("BrandingThemes").await?;
    let wrapper: BrandingThemesWrapper = serde_json::from_value(response)?;
    Ok(wrapper.branding_themes)
}

pub async fn get(client: &CachedClient, id: &str) -> Result<BrandingTheme> {
    let response = client.get(&format!("BrandingThemes/{id}")).await?;
    let wrapper: BrandingThemesWrapper = serde_json::from_value(response)?;
    wrapper
        .branding_themes
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Branding theme not found"))
}
