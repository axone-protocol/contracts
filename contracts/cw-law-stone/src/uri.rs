use url::Url;
use crate::ContractError;
use crate::ContractError::{DependencyUri, NotImplemented};
use crate::state::Object;

// const COSMWASM_SCHEME = "cosmwasm";

pub fn uri_to_object(uri: String) -> Result<Object, ContractError> {
    // let url = Url::parse(uri.as_str()).map_err(|e| ContractError::DependencyUri { error: e.into(), uri })?;
    // if url.scheme() != COSMWASM_SCHEME {
    //     Err(DependencyUri { error: })
    // }
    Err(NotImplemented {})
}
