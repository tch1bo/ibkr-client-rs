use super::components::error::set_global_error_message;
use super::models::Account;
use super::models::GlobalStateContext;
use anyhow::{bail, Context, Result};
use gloo_net::http::Request;
use log;
use serde::de::DeserializeOwned;
use yew::prelude::*;

fn make_url(endpoint: &str) -> String {
    format!("/v1/api/{}", endpoint)
}

async fn get<T: DeserializeOwned>(endpoint: &str) -> Result<T> {
    let url = make_url(endpoint);
    log::info!("requesting {:?}", url);
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

pub fn make_get_positions_url(account_id: &str) -> String {
    format!("/portfolio/{}/positions", account_id)
}

pub fn make_get_ledger_url(account_id: &str) -> String {
    format!("/portfolio/{}/ledger", account_id)
}

pub fn make_state_updater_fn_for_get_request<UrlFnT: 'static, ModelT: 'static>(
    global_state: GlobalStateContext,
    state_handle: UseStateHandle<ModelT>,
    url_fn: UrlFnT,
) -> impl FnOnce()
where
    UrlFnT: FnOnce(&str) -> String,
    ModelT: DeserializeOwned,
{
    || {
        let account_id = global_state.account_id.clone();
        if global_state.account_id.is_none() {
            return;
        }
        let account_id = account_id.unwrap();
        let url = url_fn(account_id.as_str());

        wasm_bindgen_futures::spawn_local(async move {
            match get(url.as_str()).await {
                Ok(fetched) => {
                    state_handle.set(fetched);
                }
                Err(e) => set_global_error_message(global_state, e),
            }
        });
    }
}

// TODO: interesting endpoints:
//  - iserver/account/trades
//  - portfolio/U4009416/positions/0
//  - portfolio/U4009416/cash
//  - pa/transactions
