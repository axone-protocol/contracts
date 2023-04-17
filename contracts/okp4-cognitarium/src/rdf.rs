use cosmwasm_std::StdError;

pub fn explode_iri(iri: &str) -> Result<(&str, &str), StdError> {
    for delim in ['#', '/'] {
        if let Some(index) = iri.rfind(delim) {
            return Ok((&iri[..index], &iri[index..]));
        }
    }

    Err(StdError::generic_err("Couldn't extract IRI namespace"))
}
