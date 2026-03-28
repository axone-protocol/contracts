use crate::{
    contract::{AxoneVc, AxoneVcResult},
    domain::Authority,
    msg::AxoneVcInstantiateMsg,
    state::{AUTHORITY, DEFAULT_FOO_VALUE, FOO},
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

#[allow(clippy::unnecessary_wraps)]
pub fn instantiate_handler(
    deps: DepsMut<'_>,
    env: Env,
    _info: MessageInfo,
    module: AxoneVc,
    _msg: AxoneVcInstantiateMsg,
) -> AxoneVcResult {
    let authority = Authority::new(
        &env.block.chain_id,
        module.load_state(deps.storage)?.account.addr(),
    )?;
    AUTHORITY.save(deps.storage, &authority)?;
    FOO.save(deps.storage, &DEFAULT_FOO_VALUE.to_string())?;

    Ok(module.custom_response(
        "instantiate",
        vec![
            ("authority".to_string(), authority.did().to_string()),
            ("foo".to_string(), DEFAULT_FOO_VALUE.to_string()),
        ],
    ))
}
