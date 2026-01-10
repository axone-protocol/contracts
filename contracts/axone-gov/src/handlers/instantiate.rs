use crate::{
    contract::{AxoneGov, AxoneGovResult},
    guards,
    msg::AxoneGovInstantiateMsg,
    state::CONSTITUTION,
};
use abstract_app::sdk::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn instantiate_handler(
    deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovInstantiateMsg,
) -> AxoneGovResult {
    guards::constitution(&*deps.querier, &msg.constitution)?;

    CONSTITUTION.save(deps.storage, &msg.constitution)?;

    Ok(module.response("instantiate"))
}
