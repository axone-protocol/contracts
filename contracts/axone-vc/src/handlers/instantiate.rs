use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::AxoneVcInstantiateMsg,
    services::initialize_authority,
    RESPONSE_KEY_AUTHORITY,
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
    let account = module.load_state(deps.storage)?;
    let authority =
        initialize_authority(deps.storage, &env.block.chain_id, account.account.addr())?;

    Ok(module.custom_response(
        "instantiate",
        vec![(
            RESPONSE_KEY_AUTHORITY.to_string(),
            authority.did().to_string(),
        )],
    ))
}
