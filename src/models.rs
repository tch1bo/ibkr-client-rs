use serde::Deserialize;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

#[derive(Clone, PartialEq, Deserialize, Debug)]
// Corresponds to the `account` model.
pub struct Account {
    pub id: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
// Corresponds to the `position` model.
pub struct Position {
    pub conid: u32,
    #[serde(rename = "contractDesc")]
    pub contract_desc: String,
    pub currency: String,
    #[serde(rename = "position")]
    pub num_shares: f32,
    #[serde(rename = "mktPrice")]
    pub market_price: f32,
    #[serde(rename = "mktValue")]
    pub market_value: f32,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LedgerItem {
    #[serde(rename = "settledcash")]
    pub cash: f32,
    pub currency: String,
    #[serde(rename = "netliquidationvalue")]
    pub net_liquidation_value: f32,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
// Corresponds to the `ledger` model.
pub struct Ledger {
    // A map from currency to a `LedgerItem`.
    #[serde(flatten)]
    pub items: HashMap<String, LedgerItem>,
}

impl Default for Ledger {
    fn default() -> Ledger {
        Ledger {
            items: HashMap::default(),
        }
    }
}

pub enum GlobalStateAction {
    SetErrorMessage(String),
    SetAccountId(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlobalState {
    // The error message to be displayed to the user.
    pub error_message: Option<String>,
    // The id of the user's account.
    pub account_id: Option<String>,
}

impl Default for GlobalState {
    fn default() -> GlobalState {
        GlobalState {
            error_message: None,
            account_id: None,
        }
    }
}

pub type GlobalStateContext = UseReducerHandle<GlobalState>;

impl Reducible for GlobalState {
    type Action = GlobalStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state: Self = (*self).clone();
        match action {
            GlobalStateAction::SetErrorMessage(message) => new_state.error_message = Some(message),
            GlobalStateAction::SetAccountId(account_id) => new_state.account_id = Some(account_id),
        };
        new_state.into()
    }
}
