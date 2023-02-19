use super::models::{Account, Ledger, Position};
use anyhow::{bail, Context, Result};
use gloo_net::http::Request;
use log;
use serde::de::DeserializeOwned;

fn make_url(endpoint: &str) -> String {
    format!("/v1/api/{}", endpoint)
}

async fn get<T: DeserializeOwned>(endpoint: &str) -> Result<T> {
    let url = make_url(endpoint);
    Ok(Request::get(url.as_str())
        .send()
        .await
        .with_context(|| format!("failed sending a GET request to: {}", url))?
        .json()
        .await
        .with_context(|| format!("failed deserializing JSON response from: {}", url))?)
}

pub async fn get_account() -> Result<Account> {
    let mut accounts: Vec<Account> = get("/portfolio/accounts").await?;
    if accounts.len() != 1 {
        bail!("Expected one account, got {}", accounts.len());
    }
    Ok(accounts.swap_remove(0))
}

pub async fn get_all_positions(account_id: &str) -> Result<Vec<Position>> {
    let url = format!("/portfolio/{}/positions", account_id);
    Ok(get::<Vec<Position>>(url.as_str()).await?)
}

pub async fn get_ledger(account_id: &str) -> Result<Ledger> {
    let url = format!("/portfolio/{}/ledger", account_id);
    Ok(get::<Ledger>(url.as_str()).await?)
}

// TODO: interesting endpoints:
//  - iserver/account/trades
//  - portfolio/U4009416/positions/0
//  - portfolio/U4009416/cash
//  - pa/transactions
