use std::ops::Add;
use std::os::macos::raw::stat;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint64, StdError};
use cw2::set_contract_version;
use cw_utils::Scheduled;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, VoteResponse};
use crate::msg::ExecuteMsg::vote_reset;
use crate::state::{State, STATE, Vote, VOTE_BOX_SEQ, VOTE_BOX_LIST};

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

        Ok(Response::new().add_attribute("method", "instantiate").add_attribute("yes_count", "0").add_attribute("no_count", "0"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::create_vote_box {deadline, owner} => create_vote_box(deps, env, info, deadline, owner),
        ExecuteMsg::vote_yes {id} => vote_yes(deps, env, id),
        ExecuteMsg::vote_no {id} => vote_no(deps, env, id),
        ExecuteMsg::vote_reset {id} => reset(deps, env, info, id),
    }
}

pub fn create_vote_box(deps: DepsMut, env: Env, info: MessageInfo, deadline: Scheduled, owner: String) -> Result<Response, ContractError>{
    let check = deps.api.addr_validate(&owner)?;

    let id = VOTE_BOX_SEQ.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;

    let mut new_vote_box = Vote {
        id,
        yes_count: Uint128::zero(),
        no_count: Uint128::zero(),
        deadline: deadline.clone(),
        owner : owner.clone(),
    };

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &new_vote_box)?;
    Ok(Response::new()
        .add_attribute("create_vote", "success")
        .add_attribute("print_id", id)
        .add_attribute("owner", owner.clone()))

}

pub fn vote_yes(deps: DepsMut, env: Env, id: Uint64) -> Result<Response, ContractError> {
    let mut param: Uint128 = Uint128::zero();

    let mut vote_box = VOTE_BOX_LIST.load(deps.storage, id.u64())?;

    if vote_box.deadline.is_triggered(&env.block) {
        return Err(ContractError::Expired {});
    }
    vote_box.yes_count = vote_box.yes_count.checked_add(Uint128::new(1))?;
    param = vote_box.yes_count;

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &vote_box);

    Ok(Response::new().add_attribute("method", "vote_yes").add_attribute("yes_count", param))
}

pub fn vote_no(deps: DepsMut, env: Env, id: Uint64) -> Result<Response, ContractError> {
    let mut param: Uint128 = Uint128::zero();

    let mut vote_box = VOTE_BOX_LIST.load(deps.storage, id.u64())?;

    if vote_box.deadline.is_triggered(&env.block) {
        return Err(ContractError::Expired {});
    }

    vote_box.no_count = vote_box.no_count.checked_add(Uint128::new(1))?;
    param = vote_box.no_count;

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &vote_box);

    Ok(Response::new().add_attribute("method", "vote_yes").add_attribute("yes_count", param))
}

pub fn reset(deps: DepsMut, env: Env, info: MessageInfo, id: Uint64) -> Result<Response, ContractError> {

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
        .add_attribute("no_count", param2))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<VoteResponse> {
    match msg {
        QueryMsg::query_vote => query_vote(deps),
    }
}

pub fn query_vote(deps: Deps) -> StdResult<VoteResponse> {
    let vote_item = STATE.load(deps.storage)?;
    // Ok(VoteResponse {
    //     vote: vote_item
    // })
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use cw_utils::Scheduled;

    // #[test]
    // fn proper_initialization() {
    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
    //     let msg = InstantiateMsg { deadline: Scheduled::AtHeight(123) };
    //     let info = mock_info("admin", &coins(1000, "earth"));
    //     let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    //     let value = res.attributes;
    //     assert_eq!("0", value[1].value);
    // }
    //
    #[test]
    fn create() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let intmsg = InstantiateMsg { deadline: Scheduled::AtHeight(123111) };
        let msg = ExecuteMsg::create_vote_box {deadline: intmsg.deadline, owner: "simon".to_string()};
        let intinfo = mock_info("admin", &coins(1000, "earth"));
        let info = mock_info("admin", &coins(1000, "earth"));
        let intres = instantiate(deps.as_mut(), mock_env(), intinfo, intmsg).unwrap();
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let value = res.attributes;
        assert_eq!("1", value[1].value);
        assert_eq!("simon", value[2].value);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let intmsg = InstantiateMsg { deadline: Scheduled::AtHeight(123111) };
        let msg = ExecuteMsg::vote_yes {id: Uint64::new(1)};
        let intinfo = mock_info("admin", &coins(1000, "earth"));
        let info = mock_info("admin", &coins(1000, "earth"));
        let intres = instantiate(deps.as_mut(), mock_env(), intinfo, intmsg).unwrap();
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        // let value = res.attributes;
        // assert_eq!("0", value[1].value, "initial value is {}", value[1].value);
    }

    // #[test]
    // fn decrement() {
    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
    //     let intmsg = InstantiateMsg { deadline: Scheduled::AtHeight(123111) };
    //     let msg = ExecuteMsg::vote_no{id: Uint64::new(1)};
    //     let intinfo = mock_info("admin", &coins(1000, "earth"));
    //     let info = mock_info("admin", &coins(1000, "earth"));
    //     let intres = instantiate(deps.as_mut(), mock_env(), intinfo, intmsg).unwrap();
    //     execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    //     let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    //     let value = res.attributes;
    //     assert_eq!("2", value[1].value);
    // }
    //
    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
    //     let intmsg = InstantiateMsg { deadline: Scheduled::AtHeight(123111) };
    //     let msg = ExecuteMsg::vote_reset {id: Uint64::new(1)};
    //     let intinfo = mock_info("admin", &coins(1000, "earth"));
    //     let info = mock_info("admin", &coins(1000, "earth"));
    //     let intres = instantiate(deps.as_mut(), mock_env(), intinfo, intmsg).unwrap();
    //     let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    //     let value = res.attributes;
    //     assert_eq!("0", value[1].value);
    //     assert_eq!("0", value[2].value);
    // }
    //
    // #[test]
    // fn query_test() {
    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
    //     let intmsg = InstantiateMsg { deadline: Scheduled::AtHeight(123111) };
    //     let msg = QueryMsg::query_vote;
    //     let intinfo = mock_info("admin", &coins(1000, "earth"));
    //     let info = mock_info("admin", &coins(1000, "earth"));
    //     let intres = instantiate(deps.as_mut(), mock_env(), intinfo, intmsg).unwrap();
    //     let res = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
    // }
}
