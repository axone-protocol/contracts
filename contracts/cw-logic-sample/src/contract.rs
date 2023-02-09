#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use logic_bindings::{AskResponse, LogicCustomQuery};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:logic-sample-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<LogicCustomQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        program: msg.program,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<LogicCustomQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ask { query } => to_binary(&query::ask(deps, query)?),
    }
}

pub mod query {
    use super::*;

    pub fn ask(deps: Deps<LogicCustomQuery>, query: String) -> StdResult<AskResponse> {
        let state = STATE.load(deps.storage)?;

        let req = LogicCustomQuery::Ask {
            program: state.program,
            query,
        }
        .into();

        deps.querier.query(&req)
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{Coin, coins, from_binary, OwnedDeps};
    use logic_bindings::testing::mock::mock_dependencies_with_logic_and_balance;


    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_logic_and_balance(&[Coin::new(10000, "uknow".to_string())]);

        let msg = InstantiateMsg {
            program: "bank_balances_has_coin(A, D, V, S) :- bank_balances(A, R), member(D-V, R), compare(>, V, S).".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's check if logic querier is called to answer to the `Ask` query.
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Ask { query: "".to_string() }).unwrap();
        let value: AskResponse = from_binary(&res).unwrap();
        assert_eq!(true, value.answer.unwrap().success);
    }
}
