use crate::cache::CachedClient;
use crate::error::Result;
use crate::models::report::{Report, ReportsWrapper};

pub async fn profit_and_loss(
    client: &CachedClient,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Report> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(from) = from {
        params.push(("fromDate", from));
    }
    if let Some(to) = to {
        params.push(("toDate", to));
    }
    fetch_report(client, "Reports/ProfitAndLoss", &params).await
}

pub async fn balance_sheet(client: &CachedClient, date: Option<&str>) -> Result<Report> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(date) = date {
        params.push(("date", date));
    }
    fetch_report(client, "Reports/BalanceSheet", &params).await
}

pub async fn trial_balance(client: &CachedClient, date: Option<&str>) -> Result<Report> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(date) = date {
        params.push(("date", date));
    }
    fetch_report(client, "Reports/TrialBalance", &params).await
}

pub async fn bank_summary(client: &CachedClient) -> Result<Report> {
    fetch_report(client, "Reports/BankSummary", &[]).await
}

pub async fn budget_summary(
    client: &CachedClient,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Report> {
    let mut params: Vec<(&str, &str)> = Vec::new();
    if let Some(from) = from {
        params.push(("date", from));
    }
    if let Some(to) = to {
        params.push(("timeframe", to));
    }
    fetch_report(client, "Reports/BudgetSummary", &params).await
}

pub async fn executive_summary(client: &CachedClient) -> Result<Report> {
    fetch_report(client, "Reports/ExecutiveSummary", &[]).await
}

pub async fn aged_receivables(client: &CachedClient, contact_id: &str) -> Result<Report> {
    fetch_report(
        client,
        "Reports/AgedReceivablesByContact",
        &[("contactID", contact_id)],
    )
    .await
}

pub async fn aged_payables(client: &CachedClient, contact_id: &str) -> Result<Report> {
    fetch_report(
        client,
        "Reports/AgedPayablesByContact",
        &[("contactID", contact_id)],
    )
    .await
}

async fn fetch_report(
    client: &CachedClient,
    path: &str,
    params: &[(&str, &str)],
) -> Result<Report> {
    let response = client.get_with_params(path, params).await?;
    let wrapper: ReportsWrapper = serde_json::from_value(response)?;
    wrapper
        .reports
        .into_iter()
        .next()
        .ok_or_else(|| crate::error::XeroCliError::api(404, "Report not found"))
}
