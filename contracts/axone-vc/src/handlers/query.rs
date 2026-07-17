use crate::{
    contract::{AxoneVc, AxoneVcResult},
    msg::{
        AuthorityResponse, AxoneVcQueryMsg, CredentialRawResponse, CredentialResponse,
        Quad as QuadResponse, VerifyCredentialResponse,
    },
    services::{authority, credential, credential_raw, verify_credential},
    translation::DecodedQuad,
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env};
use oxrdf::GraphName;

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
        AxoneVcQueryMsg::Credential { identifier } => {
            to_json_binary(&query_credential(deps, identifier)?)
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

fn query_credential(deps: Deps<'_>, identifier: String) -> AxoneVcResult<CredentialResponse> {
    let result = credential(deps.storage, &identifier)?;
    Ok(CredentialResponse {
        identifier: result.parsed.id().clone(),
        types: result.parsed.types().clone(),
        issuer: result.parsed.issuer().clone(),
        subject: result.parsed.subject_id().clone(),
        valid_from: result.parsed.valid_from(),
        valid_until: result.parsed.valid_until(),
        quads: result.quads.into_iter().map(QuadResponse::from).collect(),
    })
}

fn query_authority(deps: Deps<'_>) -> AxoneVcResult<AuthorityResponse> {
    let authority = authority(deps.storage)?;
    Ok(AuthorityResponse {
        did: authority.did().to_string(),
    })
}

impl From<DecodedQuad> for QuadResponse {
    fn from(quad: DecodedQuad) -> Self {
        Self {
            subject: quad.subject.to_string(),
            predicate: quad.predicate.to_string(),
            object: quad.object.to_string(),
            graph_name: match quad.graph_name {
                GraphName::DefaultGraph => None,
                graph_name => Some(graph_name.to_string()),
            },
        }
    }
}
