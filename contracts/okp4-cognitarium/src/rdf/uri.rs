use cosmwasm_std::{StdError, StdResult};

use crate::msg::Prefix;

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
        return Ok((iri[..index + 1].to_string(), iri[index + 1..].to_string()));
    }

    Err(StdError::generic_err("Couldn't extract IRI namespace"))
}

// Expand a compacted URI (CURIE - URI with prefix) to a full URI.
pub fn expand_uri<'a>(curie: String, prefixes: &Vec<Prefix>) -> StdResult<String> {
    let idx = curie
        .rfind(':')
        .ok_or_else(|| StdError::generic_err(format!("Malformed CURIE: {}", curie)))?;

    let prefix = curie[..idx].to_string();
    let suffix = curie[idx + 1..].to_string();

    let namespace = &prefixes
        .iter()
        .find(|p| p.prefix == prefix)
        .ok_or_else(|| StdError::generic_err(format!("Prefix not found: {}", prefix)))?
        .namespace;

    Ok(format!("{}{}", namespace, suffix))
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
            explode_iri("https://ontology.okp4.space/core/Governance"),
            Ok((
                "https://ontology.okp4.space/core/".to_string(),
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
        let prefixes = vec![
            Prefix {
                prefix: "ex".to_string(),
                namespace: "http://example.com/".to_string(),
            },
            Prefix {
                prefix: "rdf".to_string(),
                namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            },
        ];

        assert_eq!(
            expand_uri("ex:resource".to_string(), &prefixes),
            Ok("http://example.com/resource".to_string())
        );

        assert_eq!(
            expand_uri("ex:".to_string(), &prefixes),
            Ok("http://example.com/".to_string())
        );

        assert_eq!(
            expand_uri("unknown:resource".to_string(), &prefixes),
            Err(StdError::generic_err("Prefix not found: unknown"))
        );

        assert_eq!(
            expand_uri("malformed_curie:".to_string(), &prefixes),
            Err(StdError::generic_err("Prefix not found: malformed_curie"))
        );

        assert_eq!(
            expand_uri("malformed_curie".to_string(), &prefixes),
            Err(StdError::generic_err("Malformed CURIE: malformed_curie"))
        );
    }
}
