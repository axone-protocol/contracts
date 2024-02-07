#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    instantiate2_address, to_json_binary, Binary, CodeInfoResponse, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Dataverse, DATAVERSE};

// version info for migration info
const CONTRACT_NAME: &str = concat!("crates.io:", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let creator = deps.api.addr_canonicalize(env.contract.address.as_str())?;
    let CodeInfoResponse { checksum, .. } = deps
        .querier
        .query_wasm_code_info(msg.triplestore_config.code_id.u64())?;
    let salt = Binary::from(msg.name.as_bytes());

    let _triplestore_address = instantiate2_address(&checksum, &creator, &salt)?;

    // Necessary stuff for testing purposes, see: https://github.com/CosmWasm/cosmwasm/issues/1648
    let triplestore_address = {
        #[cfg(not(test))]
        {
            deps.api.addr_humanize(&_triplestore_address)?
        }
        #[cfg(test)]
        cosmwasm_std::Addr::unchecked("predicted address")
    };

    DATAVERSE.save(
        deps.storage,
        &Dataverse {
            name: msg.name.clone(),
            triplestore_address: triplestore_address.clone(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("triplestore_address", triplestore_address.to_string())
        .add_message(WasmMsg::Instantiate2 {
            admin: Some(env.contract.address.to_string()),
            code_id: msg.triplestore_config.code_id.u64(),
            label: format!("{}_triplestore", msg.name),
            msg: to_json_binary(&okp4_cognitarium::msg::InstantiateMsg {
                limits: msg.triplestore_config.limits.into(),
            })?,
            funds: vec![],
            salt,
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SubmitClaims {
            metadata,
            format: _,
        } => execute::submit_claims(deps, metadata),
        _ => Err(StdError::generic_err("Not implemented").into()),
    }
}

pub mod execute {
    use super::*;
    use crate::credential::vc::VerifiableCredential;
    use okp4_rdf::dataset::Dataset;
    use okp4_rdf::serde::NQuadsReader;
    use std::io::BufReader;

    pub fn submit_claims(deps: DepsMut<'_>, data: Binary) -> Result<Response, ContractError> {
        let buf = BufReader::new(data.as_slice());
        let mut reader = NQuadsReader::new(buf);
        let rdf_quads = reader.read_all()?;
        let vc_dataset = Dataset::from(rdf_quads.as_slice());
        let vc = VerifiableCredential::try_from(&vc_dataset)?;
        vc.verify(deps)?;

        Ok(Response::default())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<'_>, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Not implemented"))
}

pub mod query {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{TripleStoreConfig, TripleStoreLimitsInput};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{
        Addr, Attribute, ContractResult, HexBinary, SubMsg, SystemError, SystemResult, Uint128,
        Uint64, WasmQuery,
    };

    #[test]
    fn proper_instantiate() {
        let mut deps = mock_dependencies();
        deps.querier.update_wasm(|query| match query {
            WasmQuery::CodeInfo { code_id, .. } => {
                let resp = CodeInfoResponse::new(
                    code_id.clone(),
                    "creator".to_string(),
                    HexBinary::from_hex(
                        "3B94AAF0B7D804B5B458DED0D20CACF95D2A1C8DF78ED3C89B61291760454AEC",
                    )
                    .unwrap(),
                );
                SystemResult::Ok(ContractResult::Ok(to_json_binary(&resp).unwrap()))
            }
            _ => SystemResult::Err(SystemError::Unknown {}),
        });

        let store_limits = TripleStoreLimitsInput {
            max_byte_size: Some(Uint128::from(50000u128)),
            ..Default::default()
        };

        let msg = InstantiateMsg {
            name: "my-dataverse".to_string(),
            triplestore_config: TripleStoreConfig {
                code_id: Uint64::from(17u64),
                limits: store_limits.clone(),
            },
        };

        let env = mock_env();
        let res = instantiate(deps.as_mut(), env.clone(), mock_info("creator", &[]), msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![Attribute::new("triplestore_address", "predicted address")]
        );
        assert_eq!(
            res.messages,
            vec![SubMsg::new(WasmMsg::Instantiate2 {
                admin: Some(env.contract.address.to_string()),
                code_id: 17,
                label: "my-dataverse_triplestore".to_string(),
                msg: to_json_binary(&okp4_cognitarium::msg::InstantiateMsg {
                    limits: store_limits.into(),
                })
                .unwrap(),
                funds: vec![],
                salt: Binary::from("my-dataverse".as_bytes()),
            })]
        );
        assert_eq!(
            DATAVERSE.load(&deps.storage).unwrap(),
            Dataverse {
                name: "my-dataverse".to_string(),
                triplestore_address: Addr::unchecked("predicted address"),
            }
        )
    }
}
