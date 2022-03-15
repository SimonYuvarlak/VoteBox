use crate::state::Vote;
use cosmwasm_std::{Uint128, Uint64};
use cw_utils::Scheduled;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum ExecuteMsg {
    create_vote_box {
        deadline: Scheduled,
        owner: String,
        topic: String,
        native_denom: Option<String>,
    },
    vote {
        id: Uint64,
        vote: i32,
    },
    vote_reset {
        id: Uint64,
    },
    vote_remove {
        id: Uint64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum QueryMsg {
    query_vote {
        id: Uint64,
    },
    get_list {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    get_votebox_count {},
    get_vbop_count {},
    get_voteboxes_by_owner {
        owner: String,
    },
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct VoteResponse {
    pub id: Uint64,
    pub yes_count: Uint128,
    pub no_count: Uint128,
    pub abstain_count: Uint128,
    pub deadline: Scheduled,
    pub owner: String,
    pub topic: String,
    pub native: Option<String>,
    pub total_amount: Uint128,
}

impl Into<VoteResponse> for Vote {
    fn into(self) -> VoteResponse {
        VoteResponse {
            id: self.id,
            owner: self.owner,
            yes_count: self.yes_count,
            no_count: self.no_count,
            abstain_count: self.abstain_count,
            deadline: self.deadline,
            topic: self.topic,
            native: self.native_denom,
            total_amount: self.total_amount,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VBCountResponse {
    pub count: Uint64,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[allow(non_snake_case)]
pub struct VoteBoxListResponse {
    pub voteList: Vec<VoteResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VBOCResponse {
    pub open: Uint64,
    pub closed: Uint64,
}
