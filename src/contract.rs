#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
    Uint128, Uint64,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use cw_utils::Scheduled;
use std::ops::Add;
//use std::os::macos::raw::stat;

use crate::error::ContractError;
use crate::msg::ExecuteMsg::vote_reset;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, VoteBoxListResponse, VoteResponse};
use crate::state::{Vote, VOTE_BOX_LIST, VOTE_BOX_SEQ};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:vote";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    VOTE_BOX_SEQ.save(deps.storage, &Uint64::zero());

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("yes_count", "0")
        .add_attribute("no_count", "0"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::create_vote_box { deadline, owner } => {
            create_vote_box(deps, env, info, deadline, owner)
        }
        ExecuteMsg::vote { id, vote } => execute_vote(deps, env, id, vote),
        ExecuteMsg::vote_reset { id } => reset(deps, env, info, id),
    }
}

pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    id: Uint64,
    vote: bool,
) -> Result<Response, ContractError> {
    let mut vote_box = VOTE_BOX_LIST.load(deps.storage, id.u64())?;
    if vote_box.deadline.is_triggered(&env.block) {
        return Err(ContractError::Expired {});
    }
    if vote {
        vote_box.yes_count = vote_box.yes_count.checked_add(Uint128::new(1))?;
    } else {
        vote_box.no_count = vote_box.no_count.checked_add(Uint128::new(1))?;
    }

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &vote_box);

    Ok(Response::new()
        .add_attribute("method", "vote given")
        .add_attribute("yes_count", vote_box.yes_count)
        .add_attribute("no count", vote_box.no_count))
}

pub fn create_vote_box(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    deadline: Scheduled,
    owner: String,
) -> Result<Response, ContractError> {
    let check = deps.api.addr_validate(&owner)?;

    let id = VOTE_BOX_SEQ.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;

    let mut new_vote_box = Vote {
        id,
        yes_count: Uint128::zero(),
        no_count: Uint128::zero(),
        deadline: deadline.clone(),
        owner: owner.clone(),
    };

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &new_vote_box)?;
    Ok(Response::new()
        .add_attribute("create_vote", "success")
        .add_attribute("print_id", id)
        .add_attribute("owner", owner.clone()))
}

pub fn reset(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: Uint64,
) -> Result<Response, ContractError> {
    let mut param1: Uint128 = Uint128::zero();
    let mut param2: Uint128 = Uint128::zero();

    let mut vote_box = VOTE_BOX_LIST.load(deps.storage, id.u64())?;

    if info.sender != vote_box.owner {
        return Err(ContractError::Unauthorized {});
    }

    if vote_box.deadline.is_triggered(&env.block) {
        return Err(ContractError::Expired {});
    }

    vote_box.yes_count = Uint128::zero();
    vote_box.no_count = Uint128::zero();
    param1 = vote_box.yes_count;
    param2 = vote_box.no_count;

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &vote_box);

    Ok(Response::new()
        .add_attribute("method", "vote_reset")
        .add_attribute("yes_count", param1)
        .add_attribute("no_count", param2)
        .add_attribute("caller", info.sender.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::query_vote { id } => to_binary(&query_vote(deps, id)?),
        QueryMsg::get_list { start_after, limit } => {
            to_binary(&query_votelist(deps, start_after, limit)?)
        }
    }
}

pub fn query_vote(deps: Deps, id: Uint64) -> StdResult<VoteResponse> {
    let vote = VOTE_BOX_LIST.load(deps.storage, id.u64())?;
    let res = VoteResponse {
        id: vote.id,
        yes_count: vote.yes_count,
        no_count: vote.no_count,
        deadline: vote.deadline,
        owner: vote.owner,
    };
    Ok(res)
}
// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn query_votelist(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<VoteBoxListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let votes: StdResult<Vec<_>> = VOTE_BOX_LIST
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();

    let res = VoteBoxListResponse {
        voteList: votes?.into_iter().map(|l| l.1.into()).collect(),
    };
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use cw_utils::Scheduled;

    /*
    #[test]
    fn proper_initialization() {
        ///Initialize
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {
            deadline: Scheduled::AtHeight(123),
        };
        let info = mock_info("admin", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let value = res.attributes;
        assert_eq!("0", value[1].value);
    }

    #[test]
    fn execution_test() {
        ///Initialize create, increment and reset
        ///Initialize
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {
            deadline: Scheduled::AtHeight(123111),
        };
        let info = mock_info("admin", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        let value = res.attributes;
        assert_eq!("0", value[1].value);
        ///Create
        let msg = ExecuteMsg::create_vote_box {
            deadline: msg.deadline,
            owner: "simon".to_string(),
        };
        let info = mock_info("admin", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let value = res.attributes;
        assert_eq!("1", value[1].value);
        assert_eq!("simon", value[2].value);
        ///Increment
        let msg = ExecuteMsg::vote {
            id: Uint64::new(1),
            vote: true,
        };
        let info = mock_info("admin", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let value = res.attributes;
        assert_eq!("1", value[1].value, "Value is {}", value[1].value);
        ///Decrement
        let msg = ExecuteMsg::vote {
            id: Uint64::new(1),
            vote: false,
        };
        let info = mock_info("admin", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let value = res.attributes;
        assert_eq!("1", value[2].value, "Value is {}", value[1].value);
        /// Query Vote
        let msgQuery = QueryMsg::query_vote { id: Uint64::new(1) };
        let res = query(deps.as_ref(), mock_env(), msgQuery.clone()).unwrap();

        ///Reset
        let msg = ExecuteMsg::vote_reset { id: Uint64::new(1) };
        let info = mock_info("simon", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let value = res.attributes;
        assert_eq!("0", value[1].value, "Value is {}", value[1].value);
        assert_eq!("0", value[2].value, "Value is {}", value[2].value);
        assert_eq!("simon", value[3].value, "Value is {}", value[3].value);
    }

     */
}
