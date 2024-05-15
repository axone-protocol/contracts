use cosmwasm_std::{StdError, StdResult};
use std::collections::HashMap;

/// Explode a compacted URI (CURIE - URI with prefix) separating it from its prefix.
pub fn explode_iri(iri: &str) -> StdResult<(String, String)> {
    let mut marker_index: Option<usize> = None;
    for delim in ['#', '/', ':'] {
        if let Some(index) = iri.rfind(delim) {
            marker_index = match marker_index {
                Some(i) => Some(i.max(index)),
                None => Some(index),
            }
        }
    }

    if let Some(index) = marker_index {
        return Ok((iri[..=index].to_string(), iri[index + 1..].to_string()));
    }

    Err(StdError::generic_err("Couldn't extract IRI namespace"))
}

/// Expand a compacted URI (CURIE - URI with prefix) to a full URI.
pub fn expand_uri(curie: &str, prefixes: &HashMap<String, String>) -> StdResult<String> {
    let idx = curie
        .rfind(':')
        .ok_or_else(|| StdError::generic_err(format!("Malformed CURIE: {curie}")))?;

    let prefix = curie[..idx].to_string();
    let namespace = prefixes
        .get(&prefix)
        .ok_or_else(|| StdError::generic_err(format!("Prefix not found: {prefix}")))?;
    let suffix = curie[idx + 1..].to_string();

    Ok(format!("{namespace}{suffix}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_explode_iri() {
        assert_eq!(
            explode_iri("http://www.w3.org/2001/XMLSchema#dateTime"),
            Ok((
                "http://www.w3.org/2001/XMLSchema#".to_string(),
                "dateTime".to_string()
            ))
        );
        assert_eq!(
            explode_iri("https://ontology.axone.space/core/Governance"),
            Ok((
                "https://ontology.axone.space/core/".to_string(),
                "Governance".to_string()
            ))
        );
        assert_eq!(
            explode_iri(
                "did:key:0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655"
            ),
            Ok((
                "did:key:".to_string(),
                "0x04d1f1b8f8a7a28f9a5a254c326a963a22f5a5b5d5f5e5d5c5b5a5958575655".to_string()
            ))
        );
        assert_eq!(
            explode_iri("wow:this/is#weird"),
            Ok(("wow:this/is#".to_string(), "weird".to_string()))
        );
        assert_eq!(
            explode_iri("this#is:weird/too"),
            Ok(("this#is:weird/".to_string(), "too".to_string()))
        );
        assert_eq!(
            explode_iri("this_doesn't_work"),
            Err(StdError::generic_err("Couldn't extract IRI namespace"))
        );
    }

    #[test]
    fn test_expand_uri() {
        let prefixes = HashMap::from([
            ("ex".to_string(), "http://example.com/".to_string()),
            (
                "rdf".to_string(),
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ),
        ]);

        assert_eq!(
            expand_uri("ex:resource", &prefixes),
            Ok("http://example.com/resource".to_string())
        );

        assert_eq!(
            expand_uri("ex:", &prefixes),
            Ok("http://example.com/".to_string())
        );

        assert_eq!(
            expand_uri("unknown:resource", &prefixes),
            Err(StdError::generic_err("Prefix not found: unknown"))
        );

        assert_eq!(
            expand_uri("malformed_curie:", &prefixes),
            Err(StdError::generic_err("Prefix not found: malformed_curie"))
        );

        assert_eq!(
            expand_uri("malformed_curie", &prefixes),
            Err(StdError::generic_err("Malformed CURIE: malformed_curie"))
        );
    }
}
