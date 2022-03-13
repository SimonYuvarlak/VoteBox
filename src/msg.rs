use crate::state::Vote;
use cosmwasm_std::{Uint128, Uint64};
use cw_utils::Scheduled;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    create_vote_box { deadline: Scheduled, owner: String, topic: String },
    vote { id: Uint64, vote: bool },
    vote_reset { id: Uint64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    query_vote {
        id: Uint64,
    },
    get_list {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteResponse {
    pub id: Uint64,
    pub yes_count: Uint128,
    pub no_count: Uint128,
    pub deadline: Scheduled,
    pub owner: String,
    pub topic: String,
}

impl Into<VoteResponse> for Vote {
    fn into(self) -> VoteResponse {
        VoteResponse {
            id: self.id,
            owner: self.owner,
            yes_count: self.yes_count,
            no_count: self.no_count,
            deadline: self.deadline,
            topic: self.topic,
        }
    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteBoxListResponse {
    pub voteList: Vec<VoteResponse>,
}
