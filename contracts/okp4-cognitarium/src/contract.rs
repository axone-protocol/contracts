use crate::contract::execute::insert;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Store, STORE, TRIPLE_KEY_INCREMENT};

// version info for migration info
const CONTRACT_NAME: &str = concat!("crates.io:", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STORE.save(deps.storage, &Store::new(info.sender, msg.limits.into()))?;
    TRIPLE_KEY_INCREMENT.save(deps.storage, &Uint128::zero())?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InsertData { input } => insert(deps, input),
    }
}

pub mod execute {
    use super::*;
    use crate::error::StoreError;
    use crate::msg::DataInput;
    use crate::rdf;
    use crate::state::{triples, Triple};

    pub fn insert(deps: DepsMut, graph: DataInput) -> Result<Response, ContractError> {
        let mut store = STORE.load(deps.storage)?;

        let mut pk = TRIPLE_KEY_INCREMENT.load(deps.storage)?;
        let old_count = store.stat.triples_count;
        rdf::parse_triples(
            graph,
            |triple| -> Result<Triple, ContractError> { Ok(triple.try_into()?) },
            |res| -> Result<(), ContractError> {
                res.and_then(|triple| {
                    pk += Uint128::one();
                    store.stat.triples_count += Uint128::one();

                    store
                        .limits
                        .max_triple_count
                        .filter(|&max| max < store.stat.triples_count)
                        .map(|max| {
                            Err(ContractError::from(StoreError::MaxTriplesLimitExceeded(
                                max,
                            )))
                        })
                        .unwrap_or(Ok(()))?;

                    triples()
                        .save(deps.storage, pk.u128(), &triple)
                        .map_err(ContractError::Std)
                })
            },
        )?;

        TRIPLE_KEY_INCREMENT.save(deps.storage, &pk)?;
        STORE.save(deps.storage, &store)?;

        Ok(Response::new()
            .add_attribute("action", "insert")
            .add_attribute("inserted_count", store.stat.triples_count - old_count))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::StoreLimitsInput;
    use crate::state;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            limits: StoreLimitsInput {
                max_triple_count: Some(Uint128::from(1u128)),
                max_byte_size: Some(Uint128::from(2u128)),
                max_triple_byte_size: Some(Uint128::from(3u128)),
                max_query_limit: Some(4),
                max_query_variable_count: Some(5),
                max_insert_data_byte_size: Some(Uint128::from(6u128)),
                max_insert_data_triple_count: Some(Uint128::from(7u128)),
            },
        };

        let info = mock_info("owner", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let store = STORE.load(&deps.storage).unwrap();
        assert_eq!(store.owner, info.sender);
        assert_eq!(
            store.limits,
            state::StoreLimits {
                max_triple_count: Uint128::from(1u128),
                max_byte_size: Uint128::from(2u128),
                max_triple_byte_size: Uint128::from(3u128),
                max_query_limit: 4,
                max_query_variable_count: 5,
                max_insert_data_byte_size: Uint128::from(6u128),
                max_insert_data_triple_count: Uint128::from(7u128),
            }
        );
        assert_eq!(
            store.stat,
            state::StoreStat {
                triples_count: Uint128::zero(),
            }
        );

        assert_eq!(
            TRIPLE_KEY_INCREMENT.load(&deps.storage),
            Ok(Uint128::zero())
        );
    }
}
