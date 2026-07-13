use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::{AuthorityResponse, AxoneVcQueryMsg, CredentialRawResponse, VerifyCredentialResponse},
    services::{authority, credential_raw, verify_credential},
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env};

pub fn query_handler(
    deps: Deps<'_>,
    _env: Env,
    _module: &AxoneVc,
    msg: AxoneVcQueryMsg,
) -> AxoneVcResult<Binary> {
    match msg {
        AxoneVcQueryMsg::Authority {} => to_json_binary(&query_authority(deps)?),
        AxoneVcQueryMsg::VerifyCredential {
            identifier,
            valid_at,
        } => to_json_binary(&query_verify_credential(deps, identifier, valid_at)?),
        AxoneVcQueryMsg::CredentialRaw { identifier } => {
            to_json_binary(&query_credential_raw(deps, identifier)?)
        }
    }
    .map_err(Into::into)
}

fn query_verify_credential(
    deps: Deps<'_>,
    identifier: String,
    valid_at: Option<cosmwasm_std::Timestamp>,
) -> AxoneVcResult<VerifyCredentialResponse> {
    let result = verify_credential(deps.storage, &identifier, valid_at)?;
    Ok(VerifyCredentialResponse {
        exists: result.exists,
        valid: result.valid,
    })
}

fn query_credential_raw(
    deps: Deps<'_>,
    identifier: String,
) -> AxoneVcResult<CredentialRawResponse> {
    Ok(CredentialRawResponse {
        credential: credential_raw(deps.storage, &identifier)?,
    })
}

fn query_authority(deps: Deps<'_>) -> AxoneVcResult<AuthorityResponse> {
    let authority = authority(deps.storage)?;
    Ok(AuthorityResponse {
        did: authority.did().to_string(),
    })
}
