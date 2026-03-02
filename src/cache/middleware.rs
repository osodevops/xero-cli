use crate::api::XeroClient;
use crate::cache::store::CacheStore;
use crate::error::Result;
use std::sync::Arc;

pub struct CachedClient {
    inner: XeroClient,
    cache: Option<Arc<CacheStore>>,
    list_ttl: u64,
    get_ttl: u64,
}

impl CachedClient {
    pub fn new(
        client: XeroClient,
        cache: Option<Arc<CacheStore>>,
        list_ttl: u64,
        get_ttl: u64,
    ) -> Self {
        Self {
            inner: client,
            cache,
            list_ttl,
            get_ttl,
        }
    }

    pub fn without_cache(client: XeroClient) -> Self {
        Self {
            inner: client,
            cache: None,
            list_ttl: 300,
            get_ttl: 900,
        }
    }

    fn resource_type(path: &str) -> &str {
        path.split('/').next().unwrap_or(path)
    }

    fn cache_key(path: &str, params: &[(&str, &str)]) -> String {
        if params.is_empty() {
            path.to_string()
        } else {
            let mut sorted_params: Vec<_> = params.to_vec();
            sorted_params.sort_by_key(|(k, _)| *k);
            let qs: Vec<String> = sorted_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect();
            format!("{path}?{}", qs.join("&"))
        }
    }

    fn is_get_by_id(path: &str) -> bool {
        // Pattern: "Resource/some-uuid"
        let parts: Vec<&str> = path.split('/').collect();
        parts.len() >= 2 && !parts[1].is_empty()
    }

    pub async fn get(&self, path: &str) -> Result<serde_json::Value> {
        self.get_with_params(path, &[]).await
    }

    pub async fn get_with_params(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let cache = match &self.cache {
            Some(c) => c,
            None => return self.inner.get_with_params(path, params).await,
        };

        let key = Self::cache_key(path, params);
        let ttl = if Self::is_get_by_id(path) {
            self.get_ttl
        } else {
            self.list_ttl
        };

        // Check cache
        if let Some(entry) = cache.get(&key)? {
            if let Ok(value) = serde_json::from_slice(&entry.value) {
                tracing::debug!("Cache hit: {key}");
                return Ok(value);
            }
        }

        // Fetch from API
        tracing::debug!("Cache miss: {key}");
        let response = self.inner.get_with_params(path, params).await?;

        // Store in cache
        let resource_type = Self::resource_type(path);
        if let Ok(bytes) = serde_json::to_vec(&response) {
            cache.put(&key, &bytes, resource_type, ttl, None, None).ok();
        }

        Ok(response)
    }

    pub async fn put_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let response = self.inner.put_json(path, body).await?;

        // Invalidate related cache entries
        if let Some(cache) = &self.cache {
            let resource_type = Self::resource_type(path);
            cache.invalidate_by_resource(resource_type).ok();
        }

        Ok(response)
    }

    pub async fn post_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let response = self.inner.post_json(path, body).await?;

        // Invalidate related cache entries
        if let Some(cache) = &self.cache {
            let resource_type = Self::resource_type(path);
            cache.invalidate_by_resource(resource_type).ok();
        }

        Ok(response)
    }

    pub async fn delete(&self, path: &str) -> Result<serde_json::Value> {
        let response = self.inner.delete(path).await?;

        // Invalidate related cache entries
        if let Some(cache) = &self.cache {
            let resource_type = Self::resource_type(path);
            cache.invalidate_by_resource(resource_type).ok();
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_key_no_params() {
        assert_eq!(CachedClient::cache_key("Invoices", &[]), "Invoices");
    }

    #[test]
    fn cache_key_with_params() {
        let key = CachedClient::cache_key("Invoices", &[("page", "1"), ("Status", "DRAFT")]);
        assert_eq!(key, "Invoices?Status=DRAFT&page=1");
    }

    #[test]
    fn cache_key_params_sorted() {
        let key1 = CachedClient::cache_key("X", &[("b", "2"), ("a", "1")]);
        let key2 = CachedClient::cache_key("X", &[("a", "1"), ("b", "2")]);
        assert_eq!(key1, key2);
    }

    #[test]
    fn resource_type_extraction() {
        assert_eq!(CachedClient::resource_type("Invoices"), "Invoices");
        assert_eq!(CachedClient::resource_type("Invoices/abc-123"), "Invoices");
        assert_eq!(
            CachedClient::resource_type("Invoices/abc/History"),
            "Invoices"
        );
    }

    #[test]
    fn is_get_by_id_detection() {
        assert!(!CachedClient::is_get_by_id("Invoices"));
        assert!(CachedClient::is_get_by_id("Invoices/abc-123"));
        assert!(CachedClient::is_get_by_id("Invoices/abc/History"));
    }
}
