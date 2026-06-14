use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::AxoneVcExecuteMsg,
    services::issue_credential,
    RESPONSE_KEY_CREDENTIAL_ID,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneVc,
    msg: AxoneVcExecuteMsg,
) -> AxoneVcResult {
    match msg {
        AxoneVcExecuteMsg::IssueCredential { credential, format } => execute_issue_credential(
            deps,
            env,
            info,
            module,
            credential.as_slice(),
            format.unwrap_or_default(),
        ),
    }
}

fn execute_issue_credential(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneVc,
    credential: &[u8],
    format: crate::msg::CredentialInputFormat,
) -> AxoneVcResult {
    module
        .admin
        .assert_admin(deps.as_ref(), &env, &info.sender)?;

    let result = issue_credential(deps.storage, credential, format)?;

    Ok(module.custom_response(
        "issue_credential",
        vec![(RESPONSE_KEY_CREDENTIAL_ID.to_string(), result.credential_id)],
    ))
}
