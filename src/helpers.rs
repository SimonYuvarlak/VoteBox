/* use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, Uint128, Uint64,
    WasmMsg, WasmQuery,
};
use crate::msg::{ExecuteMsg, QueryMsg};

 */
use crate::state::Vote;

pub fn get_winner(votebox: Vote) -> i32 {
    let yes = votebox.yes_count;
    let no = votebox.no_count;
    let abs = votebox.abstain_count;
    let veto = votebox.no_with_veto_count;
    let mut votes_vec = vec![yes, no, abs, veto];
    votes_vec.sort();
    if votes_vec[0] == votes_vec[1] {
        4
    } else {
        if votes_vec[0] == votebox.yes_count {
            return 2;
        }
        if votes_vec[0] == votebox.no_count {
            return 0;
        }
        return if votes_vec[0] == votebox.abstain_count {
            1
        } else {
            3
        };
    }
}
