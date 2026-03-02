use crate::auth::TokenSet;
use crate::error::{Result, XeroCliError};
use std::path::PathBuf;

const SERVICE_NAME: &str = "xero-cli";
const KEYRING_USER: &str = "xero-tokens";

pub struct TokenStore {
    fallback_path: PathBuf,
}

impl TokenStore {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            fallback_path: config_dir.join("tokens.json"),
        }
    }

    pub fn save(&self, tokens: &TokenSet) -> Result<()> {
        // Try OS keychain first
        if self.save_to_keychain(tokens).is_ok() {
            tracing::debug!("Tokens saved to OS keychain");
            return Ok(());
        }

        // Fallback to file
        tracing::debug!("Keychain unavailable, saving tokens to file");
        self.save_to_file(tokens)
    }

    pub fn load(&self) -> Result<TokenSet> {
        // Try OS keychain first
        if let Ok(tokens) = self.load_from_keychain() {
            return Ok(tokens);
        }

        // Fallback to file
        self.load_from_file()
    }

    pub fn clear(&self) -> Result<()> {
        let _ = self.clear_keychain();
        let _ = self.clear_file();
        Ok(())
    }

    fn save_to_keychain(&self, tokens: &TokenSet) -> Result<()> {
        let json = serde_json::to_string(tokens)?;
        let entry = keyring::Entry::new(SERVICE_NAME, KEYRING_USER)
            .map_err(|e| XeroCliError::auth(format!("Keychain error: {e}")))?;
        entry
            .set_password(&json)
            .map_err(|e| XeroCliError::auth(format!("Failed to save to keychain: {e}")))?;
        Ok(())
    }

    fn load_from_keychain(&self) -> Result<TokenSet> {
        let entry = keyring::Entry::new(SERVICE_NAME, KEYRING_USER)
            .map_err(|e| XeroCliError::auth(format!("Keychain error: {e}")))?;
        let json = entry
            .get_password()
            .map_err(|e| XeroCliError::auth(format!("Failed to load from keychain: {e}")))?;
        let tokens: TokenSet = serde_json::from_str(&json)?;
        Ok(tokens)
    }

    fn clear_keychain(&self) -> Result<()> {
        let entry = keyring::Entry::new(SERVICE_NAME, KEYRING_USER)
            .map_err(|e| XeroCliError::auth(format!("Keychain error: {e}")))?;
        entry
            .delete_credential()
            .map_err(|e| XeroCliError::auth(format!("Failed to clear keychain: {e}")))?;
        Ok(())
    }

    fn save_to_file(&self, tokens: &TokenSet) -> Result<()> {
        if let Some(parent) = self.fallback_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(tokens)?;
        std::fs::write(&self.fallback_path, json)?;

        // Set restrictive permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&self.fallback_path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    fn load_from_file(&self) -> Result<TokenSet> {
        let json = std::fs::read_to_string(&self.fallback_path)
            .map_err(|_| XeroCliError::auth("No stored credentials found"))?;
        let tokens: TokenSet = serde_json::from_str(&json)?;
        Ok(tokens)
    }

    fn clear_file(&self) -> Result<()> {
        if self.fallback_path.exists() {
            std::fs::remove_file(&self.fallback_path)?;
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

        store.save_to_file(&tokens).unwrap();
        let loaded = store.load_from_file().unwrap();

        assert_eq!(loaded.access_token, tokens.access_token);
        assert_eq!(loaded.refresh_token, tokens.refresh_token);
        assert_eq!(loaded.tenant_id, tokens.tenant_id);
    }

    #[test]
    fn clear_file_removes_tokens() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().to_path_buf());
        let tokens = make_test_tokens();

        store.save_to_file(&tokens).unwrap();
        assert!(store.fallback_path.exists());

        store.clear_file().unwrap();
        assert!(!store.fallback_path.exists());
    }

    #[test]
    fn load_missing_file_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().to_path_buf());
        assert!(store.load_from_file().is_err());
    }
}
