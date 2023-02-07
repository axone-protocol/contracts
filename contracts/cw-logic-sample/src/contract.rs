#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use logic_bindings::{LogicCustomQuery, AskResponse};

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
        QueryMsg::Ask { query} => to_binary(&query::ask(deps, query)?),
    }
}

pub mod query {
    use super::*;

    pub fn ask(deps: Deps<LogicCustomQuery>, query: String) -> StdResult<AskResponse> {
        let state = STATE.load(deps.storage)?;

        let req = LogicCustomQuery::Ask {
            program: state.program,
            query,
        }.into();

        deps.querier.query(&req)
    }
}
