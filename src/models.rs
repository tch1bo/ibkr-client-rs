use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Account {
    id: String,
}

