use gloo_net::http::Request;

use super::models::Account;
use serde::de::DeserializeOwned;

fn make_url(endpoint: &str) -> String {
    format!("/v1/api/{}", endpoint)
}

async fn get<T: DeserializeOwned>(endpoint: &str) -> T {
    Request::get(make_url(endpoint).as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn get_account() -> Account {
    let account: Account = get("/portfolio/accounts").await;
    account
}
