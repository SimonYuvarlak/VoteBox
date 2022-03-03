use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use cw_utils::Scheduled;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub vote: Vote,
    pub deadline: Scheduled
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote {
    pub yes_count: Uint128,
    pub no_count: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
