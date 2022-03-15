use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Uint64};
use cw_storage_plus::{Item, Map};
use cw_utils::Scheduled;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote {
    pub id: Uint64,
    pub yes_count: Uint128,
    pub no_count: Uint128,
    pub abstain_count: Uint128,
    pub deadline: Scheduled,
    pub owner: String,
    pub topic: String,
    pub total_amount: Uint128,
    pub native_denom: Option<String>,
    pub voters: Vec<Addr>,
    pub voter_count: Uint128,
}

pub const VOTE_BOX_LIST: Map<u64, Vote> = Map::new("votebox list");
pub const VOTE_BOX_SEQ: Item<Uint64> = Item::new("votebox seq");
