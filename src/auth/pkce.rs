use crate::auth::{Tenant, TokenSet};
use crate::error::{Result, XeroCliError};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenResponse,
    TokenUrl,
};
const AUTH_URL: &str = "https://login.xero.com/identity/connect/authorize";
const TOKEN_URL: &str = "https://identity.xero.com/connect/token";
const CONNECTIONS_URL: &str = "https://api.xero.com/connections";

pub async fn login(client_id: &str, scopes: &[String], callback_port: u16) -> Result<TokenSet> {
    let redirect_url = format!("http://localhost:{callback_port}/callback");

    let oauth_client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_auth_uri(
            AuthUrl::new(AUTH_URL.to_string()).map_err(|e| XeroCliError::auth(e.to_string()))?,
        )
        .set_token_uri(
            TokenUrl::new(TOKEN_URL.to_string()).map_err(|e| XeroCliError::auth(e.to_string()))?,
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url.clone())
                .map_err(|e| XeroCliError::auth(e.to_string()))?,
        );

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let scope_list: Vec<oauth2::Scope> = scopes
        .iter()
        .map(|s| oauth2::Scope::new(s.clone()))
        .collect();

    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scope_list)
        .set_pkce_challenge(pkce_challenge)
        .url();

    eprintln!("Opening browser for Xero login...");
    eprintln!("If the browser doesn't open, visit: {auth_url}");
    if open::that(auth_url.as_str()).is_err() {
        eprintln!("Could not open browser automatically.");
    }

    let code = receive_callback(callback_port, csrf_token.secret())?;

    let http_client = reqwest::Client::new();
    let token_response = oauth_client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await
        .map_err(|e| XeroCliError::auth(format!("Token exchange failed: {e}")))?;

    let expires_at = chrono::Utc::now()
        + token_response
            .expires_in()
            .map(|d| chrono::Duration::seconds(d.as_secs() as i64))
            .unwrap_or(chrono::Duration::minutes(30));

    let access_token = token_response.access_token().secret().to_string();

    let tenants = fetch_tenants(&access_token).await?;
    let tenant_id = tenants.first().map(|t| t.tenant_id.clone());

    if tenants.len() > 1 {
        eprintln!("Multiple organisations found:");
        for (i, t) in tenants.iter().enumerate() {
            eprintln!(
                "  {}. {} ({})",
                i + 1,
                t.tenant_name.as_deref().unwrap_or("Unknown"),
                t.tenant_id
            );
        }
        eprintln!("Using the first organisation. Use `xero config` to set a different default.");
    }

    Ok(TokenSet {
        access_token,
        refresh_token: token_response
            .refresh_token()
            .map(|t| t.secret().to_string()),
        expires_at,
        token_type: "Bearer".to_string(),
        scopes: scopes.to_vec(),
        tenant_id,
    })
}

fn receive_callback(port: u16, expected_state: &str) -> Result<String> {
    let server = tiny_http::Server::http(format!("127.0.0.1:{port}"))
        .map_err(|e| XeroCliError::auth(format!("Failed to start callback server: {e}")))?;

    eprintln!("Waiting for callback on port {port}...");

    let request = server
        .recv()
        .map_err(|e| XeroCliError::auth(format!("Failed to receive callback: {e}")))?;

    let url = url::Url::parse(&format!("http://localhost{}", request.url()))
        .map_err(|e| XeroCliError::auth(format!("Invalid callback URL: {e}")))?;

    let params: std::collections::HashMap<_, _> = url.query_pairs().collect();

    if let Some(error) = params.get("error") {
        let desc = params
            .get("error_description")
            .map(|s| s.to_string())
            .unwrap_or_default();
        return Err(XeroCliError::auth(format!("OAuth error: {error} — {desc}")));
    }

    let state = params
        .get("state")
        .ok_or_else(|| XeroCliError::auth("Missing state parameter in callback"))?;
    if state.as_ref() != expected_state {
        return Err(XeroCliError::auth("CSRF state mismatch"));
    }

    let code = params
        .get("code")
        .ok_or_else(|| XeroCliError::auth("Missing code parameter in callback"))?
        .to_string();

    let response = tiny_http::Response::from_string(
        "<html><body><h1>Authentication successful!</h1><p>You can close this tab.</p></body></html>",
    )
    .with_header(
        "Content-Type: text/html"
            .parse::<tiny_http::Header>()
            .unwrap(),
    );
    let _ = request.respond(response);

    Ok(code)
}

async fn fetch_tenants(access_token: &str) -> Result<Vec<Tenant>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(CONNECTIONS_URL)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| XeroCliError::auth(format!("Failed to fetch tenants: {e}")))?;

    if !resp.status().is_success() {
        let body = resp
            .text()
            .await
            .unwrap_or_else(|_| String::from("(no body)"));
        return Err(XeroCliError::auth(format!(
            "Failed to fetch tenants: {body}"
        )));
    }

    let tenants: Vec<Tenant> = resp
        .json()
        .await
        .map_err(|e| XeroCliError::auth(format!("Failed to parse tenants: {e}")))?;

    Ok(tenants)
}
