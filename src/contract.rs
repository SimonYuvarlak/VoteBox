use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, VBCountResponse, VBOCResponse, VoteBoxListResponse,
    VoteResponse,
};
use crate::state::{Vote, VOTE_BOX_LIST, VOTE_BOX_SEQ};
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

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:vote";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
#[allow(unused_must_use)]
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
        ExecuteMsg::create_vote_box {
            deadline,
            owner,
            topic,
        } => create_vote_box(deps, env, info, deadline, owner, topic),
        ExecuteMsg::vote { id, vote } => execute_vote(deps, env, id, vote),
        ExecuteMsg::vote_reset { id } => reset(deps, env, info, id),
        ExecuteMsg::vote_remove { id } => remove_votebox(deps, env, info, id),
    }
}
#[allow(unused_must_use)]
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
    _env: Env,
    _info: MessageInfo,
    deadline: Scheduled,
    owner: String,
    topic: String,
) -> Result<Response, ContractError> {
    let owner = deps.api.addr_validate(&owner)?;

    let id = VOTE_BOX_SEQ.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;

    let new_vote_box = Vote {
        id,
        yes_count: Uint128::zero(),
        no_count: Uint128::zero(),
        deadline: deadline.clone(),
        owner: owner.to_string(),
        topic: topic.clone(),
    };

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &new_vote_box)?;
    Ok(Response::new()
        .add_attribute("create_vote", "success")
        .add_attribute("print_id", id)
        .add_attribute("owner", owner.clone())
        .add_attribute("topic", topic.clone()))
}

#[allow(unused_must_use)]
pub fn reset(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: Uint64,
) -> Result<Response, ContractError> {
    let mut vote_box = VOTE_BOX_LIST.load(deps.storage, id.u64())?;

    if info.sender != vote_box.owner {
        return Err(ContractError::Unauthorized {});
    }

    if vote_box.deadline.is_triggered(&env.block) {
        return Err(ContractError::Expired {});
    }
    vote_box.yes_count = Uint128::zero();
    vote_box.no_count = Uint128::zero();

    VOTE_BOX_LIST.save(deps.storage, id.u64(), &vote_box);
    Ok(Response::new()
        .add_attribute("method", "vote_reset")
        .add_attribute("yes_count", vote_box.yes_count)
        .add_attribute("no_count", vote_box.no_count)
        .add_attribute("caller", info.sender.to_string()))
}

pub fn remove_votebox(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: Uint64,
) -> Result<Response, ContractError> {
    let vote_box = VOTE_BOX_LIST.load(deps.storage, id.u64())?;
    if info.sender != vote_box.owner {
        return Err(ContractError::Unauthorized {});
    }
    if vote_box.deadline.is_triggered(&env.block) {
        return Err(ContractError::Expired {});
    }

    // alttaki satır isleyince son id bir eksildigi icin ayni id ile tekrar votebox olusturmak deneniyo
    //VOTE_BOX_SEQ.update::<_, StdError>(deps.storage, |id| Ok(id.checked_sub(Uint64::new(1))?));
    VOTE_BOX_LIST.remove(deps.storage, vote_box.id.u64());

    Ok(Response::new()
        .add_attribute("method: ", "votebox deleted")
        .add_attribute("deleted id: ", id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::query_vote { id } => to_binary(&query_vote(deps, id)?),
        QueryMsg::get_list { start_after, limit } => {
            to_binary(&query_votelist(deps, start_after, limit)?)
        }
        QueryMsg::get_votebox_count {} => to_binary(&query_votebox_count(deps)?),
        QueryMsg::get_vbop_count {} => to_binary(&query_votebox_count(deps)?),
        QueryMsg::get_voteboxes_by_owner { owner } => {
            to_binary(&query_voteboxes_by_owner(deps, owner)?)
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
        topic: vote.topic,
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

pub fn query_votebox_count(deps: Deps) -> StdResult<VBCountResponse> {
    let res = VBCountResponse {
        count: VOTE_BOX_SEQ.load(deps.storage)?,
    };
    Ok(res)
}

pub fn query_vb_open_closed(deps: Deps, env: Env) -> StdResult<VBOCResponse> {
    let votes: StdResult<Vec<_>> = VOTE_BOX_LIST
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let bisi: Vec<VoteResponse> = votes?.into_iter().map(|l| l.1.into()).collect();
    let mut open = Uint64::zero();
    let mut closed = Uint64::zero();

    for i in bisi {
        if i.deadline.is_triggered(&env.block) {
            closed += Uint64::new(1);
        } else {
            open += Uint64::new(1);
        }
    }
    let res = VBOCResponse { open, closed };
    Ok(res)
}
pub fn query_voteboxes_by_owner(deps: Deps, owner: String) -> StdResult<VoteBoxListResponse> {
    let voteboxes: StdResult<Vec<_>> = VOTE_BOX_LIST
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let vote_boxes: Vec<Vote> = voteboxes?.into_iter().map(|list| list.1).collect();
    let mut voteboxes_owned: Vec<VoteResponse> = vec![];
    for votebox in vote_boxes {
        if votebox.owner == owner {
            voteboxes_owned.push(votebox.into());
        }
    }
    let res = VoteBoxListResponse {
        voteList: voteboxes_owned,
    };
    Ok(res)
}
#[cfg(test)]
mod tests {
    /*
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
    };
    use cosmwasm_std::{coins, from_binary, QueryResponse};
    use cw_utils::Scheduled;
    use std::u64;

    */

    /*
    #[test]
    fn proper_initialization() {
        ///Initialize
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("admin", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let value = res.attributes;
        assert_eq!("0", value[1].value);
    }
    */

    /*#[test]
    fn vote_list_removal_test(){
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("test", &coins(1000, "earth"));

        ///Initialize - Create 2 and delete 1
        let msgInit = InstantiateMsg {};
        let resInit = instantiate(deps.as_mut(), mock_env(), info.clone(), msgInit).unwrap();
        let value = resInit.attributes;
        assert_eq!("0", value[1].value);

        //Create 2 voteboxes

        let msg = ExecuteMsg::create_vote_box {
            deadline: Scheduled::AtHeight(1000000000000),
            owner: "test".to_string(),
            topic: "test".to_string(),
        };

        //create votebox id 1
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        //create votebox id 2
        let res2 = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        //create votebox id 3
        execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        //Query VoteBoxes
        let resQuery: VoteResponse = query_vote(deps.as_ref(), Uint64::new(1)).unwrap();
        assert_eq!(resQuery.id, Uint64::new(1));
        // Remove Votebox id 2
        let msgRemove = ExecuteMsg::vote_remove { id: Uint64::new(2) };
        let resRemove =
            remove_votebox(deps.as_mut(), mock_env(), info.clone(), Uint64::new(2)).unwrap();

        /*let voteListSize = query_votebox_count(deps.as_ref()).unwrap();
        assert_eq!(voteListSize.count, Uint64::new(1));*/

        //try create votebox id 4
        execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        //try create votebox id 5
        execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        //list all created voteboxes
        let res: VoteBoxListResponse = query_votelist(deps.as_ref(), None, None).unwrap();
        println!("Value is {:?}", res);

        //try list votebox id 2
        /*let resQuery: VoteResponse = query_vote(deps.as_ref(), Uint64::new(2)).unwrap();
        println!("Value is {:?}", res);*/

        /*let vote = VOTE_BOX_LIST.load(deps.as_ref().storage, 2u64).unwrap();
        println!("Value is {:?}", vote);*/

    }*/
    /*#[test]
    fn query_openclosed_count_test(){
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg{};
        let info = mock_info("creator", &[]);
        let mut env = mock_env();
        env.block.height = 1;
        //instantiate votebox contract
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::create_vote_box {
            deadline: Scheduled::AtHeight(5),
            owner: "OWNER".to_string(),
            topic: "BISI".to_string()
        };
        // create votebox id 1 height 5
        execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        // create votebox id 2 height 5
        execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());

        let msg = ExecuteMsg::create_vote_box {
            deadline: Scheduled::AtHeight(6),
            owner: "OWNER".to_string(),
            topic: "BISI".to_string()
        };
        // create votebox id 3 height 6
        execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());

        let msg = ExecuteMsg::create_vote_box {
            deadline: Scheduled::AtHeight(3),
            owner: "OWNER".to_string(),
            topic: "BISI".to_string()
        };
        // create votebox id 4 height 3
        execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        // create votebox id 5 height 3
        execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        // set block height to 4
        env.block.height = 4;

        let res: VBCountResponse = query_votebox_count(deps.as_ref()).unwrap();
        println!("Value is {}", res.count);

        let res: VoteBoxListResponse = query_votelist(deps.as_ref(), None, None).unwrap();
        println!("Value is {:?}", res);

        let res: VBOCResponse = query_vb_open_closed(deps.as_ref(), env.clone()).unwrap();
        println!("All is {}, open {}, closed {}", (res.open + res.closed), res.open, res.closed);

    }*/

    /*#[test]
    fn execution_test() {
        // ///Initialize create, increment and reset
        // ///Initialize
        // let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        // let msg = InstantiateMsg {};
        // let info = mock_info("admin", &coins(1000, "earth"));
        // let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        // let value = res.attributes;
        // assert_eq!("0", value[1].value);
        // ///Create
        // let msg = ExecuteMsg::create_vote_box {
        //     deadline: Scheduled::AtHeight(1000000),
        //     owner: "simon".to_string(),
        //     topic: "trial".to_string(),
        // };
        // let info = mock_info("admin", &coins(1000, "earth"));
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        // let value = res.attributes;
        // assert_eq!("1", value[1].value);
        // assert_eq!("simon", value[2].value);
        // assert_eq!("trial", value[3].value, "topic is: {}", value[3].value);
        // ///Increment
        // let msg = ExecuteMsg::vote {
        //     id: Uint64::new(1),
        //     vote: true,
        // };
        // let info = mock_info("admin", &coins(1000, "earth"));
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        // let value = res.attributes;
        // assert_eq!("1", value[1].value, "Value is {}", value[1].value);
        // ///Decrement
        // let msg = ExecuteMsg::vote {
        //     id: Uint64::new(1),
        //     vote: false,
        // };
        // let info = mock_info("admin", &coins(1000, "earth"));
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        // let value = res.attributes;
        // assert_eq!("1", value[2].value, "Value is {}", value[1].value);
        // /// Query Vote
        // let msgQuery = QueryMsg::query_vote { id: Uint64::new(1) };
        // let res = query(deps.as_ref(), mock_env(), msgQuery.clone()).unwrap();
        //
        // ///Reset
        // let msg = ExecuteMsg::vote_reset { id: Uint64::new(1) };
        // let info = mock_info("simon", &coins(1000, "earth"));
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        // let value = res.attributes;
        // assert_eq!("0", value[1].value, "Value is {}", value[1].value);
        // assert_eq!("0", value[2].value, "Value is {}", value[2].value);
        // assert_eq!("simon", value[3].value, "Value is {}", value[3].value);
        // }
        // ///Create
        // let msg = ExecuteMsg::create_vote_box {
        //     deadline: Scheduled::AtHeight(1000000),
        //     owner: "simon".to_string(),
        //     topic: "trial".to_string(),
        // };
        // let info = mock_info("admin", &coins(1000, "earth"));
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        // let value = res.attributes;
        // assert_eq!("1", value[1].value);
        // assert_eq!("simon", value[2].value);
        // assert_eq!("trial", value[3].value, "topic is: {}", value[3].value);

    }*/
}
