use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::Constitution,
    gateway::logic::AxoneLogicQuery,
    msg::AxoneGovInstantiateMsg,
    state::save_initial_constitution,
};
use abstract_app::sdk::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, QuerierWrapper};

pub fn instantiate_handler(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovInstantiateMsg,
) -> AxoneGovResult {
    let querier = QuerierWrapper::<AxoneLogicQuery>::new(&*deps.querier);
    let constitution = Constitution::try_new(msg.constitution, &querier)?;

    let status = save_initial_constitution(deps.storage, &constitution)?;

    Ok(module.custom_response(
        "instantiate",
        vec![
            (
                "constitution_revision".to_string(),
                status.constitution_revision().to_string(),
            ),
            (
                "constitution_hash".to_string(),
                status.constitution_hash_hex(),
            ),
        ],
    ))
}
