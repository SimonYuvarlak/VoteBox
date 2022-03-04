use cosmwasm_std::{Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_utils::Scheduled;
use crate::state::{State, Vote};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub deadline: Scheduled
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    create_vote_box {deadline: Scheduled, owner: String},
    vote_yes {id: Uint64},
    vote_no {id: Uint64},
    vote_reset {id: Uint64},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    query_vote,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteResponse {
    pub vote: Vote,
}

