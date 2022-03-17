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
        description: String,
        create_date: String,
        native_denom: Option<String>,
    },
    vote {
        id: Uint64,
        vote_type: i32,
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
    get_voteboxes_by_topic {
        topic: str,
    },
    get_statistics {},
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VoteboxStatistics {
    pub total_participants: Uint128,
    pub total_voteboxes: Uint128,
    pub expired: Uint128,
    pub active: Uint128,
    pub yes_won: Uint128,
    pub no_won: Uint128,
    pub abstain_won: Uint128,
    pub no_veto_won: Uint128,
    pub total_yes_count: Uint128,
    pub total_no_count: Uint128,
    pub total_abstain_count: Uint128,
    pub total_no_veto_count: Uint128,
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteResponse {
    pub id: Uint64,
    pub yes_count: Uint128,
    pub no_count: Uint128,
    pub abstain_count: Uint128,
    pub no_with_veto_count: Uint128,
    pub deadline: Scheduled,
    pub owner: String,
    pub topic: String,
    pub description: String,
    pub create_date: String,
    pub native_denom: Option<String>,
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
            no_with_veto_count: self.no_with_veto_count,
            deadline: self.deadline,
            topic: self.topic,
            description: self.description,
            create_date: self.create_date,
            native_denom: self.native_denom,
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
