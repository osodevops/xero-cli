use crate::auth::TokenSet;
use crate::error::{Result, XeroCliError};
use std::path::PathBuf;

pub struct TokenStore {
    token_path: PathBuf,
}

impl TokenStore {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            token_path: config_dir.join("tokens.json"),
        }
    }

    pub fn save(&self, tokens: &TokenSet) -> Result<()> {
        if let Some(parent) = self.token_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(tokens)?;
        std::fs::write(&self.token_path, json)?;

        // Set restrictive permissions on Unix (owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&self.token_path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    pub fn load(&self) -> Result<TokenSet> {
        let json = std::fs::read_to_string(&self.token_path)
            .map_err(|_| XeroCliError::auth("No stored credentials found"))?;
        let tokens: TokenSet = serde_json::from_str(&json)?;
        Ok(tokens)
    }

    pub fn clear(&self) -> Result<()> {
        if self.token_path.exists() {
            std::fs::remove_file(&self.token_path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_tokens() -> TokenSet {
        TokenSet {
            access_token: "test_access_token".to_string(),
            refresh_token: Some("test_refresh_token".to_string()),
            expires_at: chrono::Utc::now() + chrono::Duration::minutes(30),
            token_type: "Bearer".to_string(),
            scopes: vec!["openid".to_string()],
            tenant_id: Some("tenant-123".to_string()),
        }
    }

    #[test]
    fn file_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().to_path_buf());
        let tokens = make_test_tokens();

        store.save(&tokens).unwrap();
        let loaded = store.load().unwrap();

        assert_eq!(loaded.access_token, tokens.access_token);
        assert_eq!(loaded.refresh_token, tokens.refresh_token);
        assert_eq!(loaded.tenant_id, tokens.tenant_id);
    }

    #[test]
    fn clear_removes_tokens() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().to_path_buf());
        let tokens = make_test_tokens();

        store.save(&tokens).unwrap();
        assert!(store.token_path.exists());

        store.clear().unwrap();
        assert!(!store.token_path.exists());
    }

    #[test]
    fn load_missing_file_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().to_path_buf());
        assert!(store.load().is_err());
    }
}
