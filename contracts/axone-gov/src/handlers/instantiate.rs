use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::Constitution,
    error::AxoneGovError,
    gateway::logic::AxoneLogicQuery,
    msg::AxoneGovInstantiateMsg,
    state::save_initial_constitution,
    AXONE_GOV_ID, RESPONSE_KEY_CONSTITUTION_HASH, RESPONSE_KEY_CONSTITUTION_REVISION,
};
use abstract_app::sdk::prelude::*;
use abstract_app::sdk::AbstractResponse;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, QuerierWrapper};

pub fn instantiate_handler(
    deps: DepsMut<'_>,
    env: Env,
    _info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovInstantiateMsg,
) -> AxoneGovResult {
    ensure_gov_uniqueness(deps.as_ref(), &env, &module)?;

    let querier = QuerierWrapper::<AxoneLogicQuery>::new(&*deps.querier);
    let constitution = Constitution::try_new(msg.constitution, &querier)?;

    let status = save_initial_constitution(deps.storage, &constitution)?;

    Ok(module.custom_response(
        "instantiate",
        vec![
            (
                RESPONSE_KEY_CONSTITUTION_REVISION.to_string(),
                status.constitution_revision().to_string(),
            ),
            (
                RESPONSE_KEY_CONSTITUTION_HASH.to_string(),
                status.constitution_hash_hex(),
            ),
        ],
    ))
}

// Abstract accounts keep a single module address per module ID.
// In other words, AXONE_GOV_ID is a singleton within an Abstract account.
//
// We still enforce it here to:
// - fail early with a domain-specific error (better UX than a generic install failure),
// - protect against misconfiguration or non-standard deployment flows,
// - keep the invariant obvious in this module's code.
fn ensure_gov_uniqueness(deps: Deps<'_>, env: &Env, module: &AxoneGov) -> AxoneGovResult<()> {
    let maybe_existing = module.modules(deps).module_address(AXONE_GOV_ID).ok();

    if let Some(existing_addr) = maybe_existing {
        if existing_addr == env.contract.address {
            return Ok(());
        }

        let account = module.load_state(deps.storage)?.account;
        return Err(AxoneGovError::ModuleAlreadyInstalled {
            module_id: AXONE_GOV_ID.to_string(),
            account: account.addr().to_string(),
            address: existing_addr.to_string(),
        });
    }

    Ok(())
}
