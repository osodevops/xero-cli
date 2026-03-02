use crate::error::{Result, XeroCliError};
use crate::rate_limit::backoff::BackoffStrategy;
use crate::rate_limit::budget::DailyBudget;
use crate::rate_limit::RateLimiter;
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

const DEFAULT_BASE_URL: &str = "https://api.xero.com/api.xro/2.0";

pub struct XeroClient {
    http: reqwest::Client,
    base_url: String,
    access_token: String,
    tenant_id: String,
    rate_limiter: Arc<RateLimiter>,
    daily_budget: Arc<DailyBudget>,
    backoff: BackoffStrategy,
}

impl XeroClient {
    pub fn new(
        access_token: String,
        tenant_id: String,
        rate_limiter: Arc<RateLimiter>,
        daily_budget: Arc<DailyBudget>,
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: std::env::var("XERO_API_BASE_URL")
                .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string()),
            access_token,
            tenant_id,
            rate_limiter,
            daily_budget,
            backoff: BackoffStrategy::default(),
        }
    }

    #[cfg(test)]
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
        );
        headers.insert(
            "xero-tenant-id",
            HeaderValue::from_str(&self.tenant_id).unwrap(),
        );
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers
    }

    pub async fn get(&self, path: &str) -> Result<serde_json::Value> {
        self.get_with_params(path, &[]).await
    }

    pub async fn get_with_params(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        self.request_with_retry(reqwest::Method::GET, path, params, None)
            .await
    }

    pub async fn put_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_with_retry(reqwest::Method::PUT, path, &[], Some(body))
            .await
    }

    pub async fn post_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_with_retry(reqwest::Method::POST, path, &[], Some(body))
            .await
    }

    pub async fn delete(&self, path: &str) -> Result<serde_json::Value> {
        self.request_with_retry(reqwest::Method::DELETE, path, &[], None)
            .await
    }

    async fn request_with_retry(
        &self,
        method: reqwest::Method,
        path: &str,
        params: &[(&str, &str)],
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));

        for attempt in 0..=self.backoff.max_retries {
            // Check daily budget
            if let Err((used, limit)) = self.daily_budget.check_and_increment() {
                return Err(XeroCliError::BudgetExhausted { used, limit });
            }

            // Acquire rate limit slot
            let _guard = self.rate_limiter.acquire().await;

            let mut request = self
                .http
                .request(method.clone(), &url)
                .headers(self.default_headers())
                .query(params);

            if let Some(body) = body {
                request = request.json(body);
            }

            let response = request.send().await?;

            // Update daily budget from response headers
            if let Some(remaining) = response
                .headers()
                .get("x-rate-limit-remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
            {
                self.daily_budget.update_from_header(remaining);
            }

            match response.status().as_u16() {
                200..=299 => {
                    let json = response.json().await?;
                    return Ok(json);
                }
                401 => {
                    return Err(XeroCliError::auth(
                        "Unauthorized — token may be expired. Run `xero auth login`.",
                    ));
                }
                403 => {
                    let body: serde_json::Value = response.json().await.unwrap_or_default();
                    let msg = body["Detail"]
                        .as_str()
                        .unwrap_or("Forbidden — check your scopes");
                    return Err(XeroCliError::api(403, msg));
                }
                429 => {
                    let retry_after = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok());

                    let delay = if let Some(secs) = retry_after {
                        self.backoff.delay_with_retry_after(secs, attempt)
                    } else {
                        self.backoff.delay_for_attempt(attempt)
                    };

                    match delay {
                        Some(d) => {
                            tracing::warn!(
                                "Rate limited (attempt {}/{}), retrying in {:?}",
                                attempt + 1,
                                self.backoff.max_retries,
                                d
                            );
                            tokio::time::sleep(d).await;
                            continue;
                        }
                        None => {
                            return Err(XeroCliError::RateLimited {
                                retry_after_secs: retry_after.unwrap_or(60),
                            });
                        }
                    }
                }
                500..=599 => {
                    if let Some(delay) = self.backoff.delay_for_attempt(attempt) {
                        tracing::warn!(
                            "Server error {}, retrying in {:?}",
                            response.status(),
                            delay
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    let body = response.text().await.unwrap_or_default();
                    return Err(XeroCliError::api(500, body));
                }
                status => {
                    let body: serde_json::Value = response.json().await.unwrap_or_default();
                    let msg = body["Message"]
                        .as_str()
                        .or_else(|| body["Detail"].as_str())
                        .unwrap_or("Unknown error")
                        .to_string();
                    return Err(XeroCliError::api(status, msg));
                }
            }
        }

        Err(XeroCliError::api(500, "Max retries exceeded".to_string()))
    }
}
