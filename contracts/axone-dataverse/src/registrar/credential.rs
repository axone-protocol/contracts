use crate::credential::rdf_marker::IRI_VC_TYPE;
use crate::credential::vc::{Claim, VerifiableCredential};
use crate::ContractError;
use cosmwasm_std::Addr;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct DataverseCredential<'a> {
    pub submitter_addr: Addr,
    pub id: &'a str,
    pub issuer: &'a str,
    pub r#type: &'a str,
    pub valid_from: &'a str,
    pub valid_until: Option<&'a str>,
    pub claim: &'a Claim<'a>,
}

impl<'a> DataverseCredential<'a> {
    fn extract_vc_type(vc: &'a VerifiableCredential<'a>) -> Result<&'a str, ContractError> {
        vc.types
            .iter()
            .filter(|t| *t != &IRI_VC_TYPE)
            .exactly_one()
            .map_err(|_| {
                ContractError::UnsupportedCredential(
                    "credential is expected to have exactly one type".to_string(),
                )
            })
            .copied()
    }

    fn extract_vc_claim(vc: &'a VerifiableCredential<'a>) -> Result<&'a Claim<'a>, ContractError> {
        vc.claims.iter().exactly_one().map_err(|_| {
            ContractError::UnsupportedCredential(
                "credential is expected to contain exactly one claim".to_string(),
            )
        })
    }
}

impl<'a> TryFrom<(Addr, &'a VerifiableCredential<'a>)> for DataverseCredential<'a> {
    type Error = ContractError;

    fn try_from(
        (submitter_addr, vc): (Addr, &'a VerifiableCredential<'a>),
    ) -> Result<Self, Self::Error> {
        Ok(DataverseCredential {
            submitter_addr,
            id: vc.id,
            issuer: vc.issuer,
            r#type: DataverseCredential::extract_vc_type(vc)?,
            valid_from: vc.issuance_date,
            valid_until: vc.expiration_date,
            claim: DataverseCredential::extract_vc_claim(vc)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutil::testutil;
    use axone_rdf::dataset::Dataset;
    use rio_api::model::{Literal, NamedNode, Quad};

    #[test]
    fn proper_from_verifiable() {
        let owned_quads = testutil::read_test_quads("vc-valid.nq");
        let dataset = Dataset::from(owned_quads.as_slice());
        let vc = VerifiableCredential::try_from(&dataset).unwrap();
        let dc_res = DataverseCredential::try_from((
            Addr::unchecked("axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0"),
            &vc,
        ));

        assert!(dc_res.is_ok());
        assert_eq!(dc_res.unwrap(), DataverseCredential {
                submitter_addr: Addr::unchecked("axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0"),
                id: "https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9",
                issuer: "did:key:zQ3shs7auhJSmVJpiUbQWco6bxxEhSqWnVEPvaBHBRvBKw6Q3",
                r#type: "https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/DigitalServiceDescriptionCredential",
                valid_from: "2024-01-22T00:00:00",
                valid_until: Some("2025-01-22T00:00:00"),
            claim: &Claim {
                id: "did:key:zQ3shhb4SvzBRLbBonsvKb3WX6WoDeKWHpsXXXMhAJETqXAfB",
                content: Dataset::new(vec![Quad {
                    subject: NamedNode {iri: "did:key:zQ3shhb4SvzBRLbBonsvKb3WX6WoDeKWHpsXXXMhAJETqXAfB"}.into(),
                    predicate: NamedNode {iri: "https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/hasCategory"}.into(),
                    object: NamedNode{iri: "https://w3id.org/axone/ontology/vnext/thesaurus/digital-service-category/Storage"}.into(),
                    graph_name: None,
                },Quad {
                    subject: NamedNode {iri: "did:key:zQ3shhb4SvzBRLbBonsvKb3WX6WoDeKWHpsXXXMhAJETqXAfB"}.into(),
                    predicate: NamedNode {iri: "https://w3id.org/axone/ontology/vnext/schema/credential/digital-service/description/hasTag"}.into(),
                    object: Literal::Simple {value: "Cloud"}.into(),
                    graph_name: None,
                }]),
            },
        })
    }

    #[test]
    fn unsupported_from_verifiable() {
        let cases = vec![
            (
                "vc-unsupported-1.nq",
                "credential is expected to have exactly one type",
            ),
            (
                "vc-unsupported-2.nq",
                "credential is expected to have exactly one type",
            ),
            (
                "vc-unsupported-3.nq",
                "credential is expected to contain exactly one claim",
            ),
        ];

        for case in cases {
            let owned_quads = testutil::read_test_quads(case.0);
            let dataset = Dataset::from(owned_quads.as_slice());
            let vc = VerifiableCredential::try_from(&dataset).unwrap();
            let dc_res = DataverseCredential::try_from((
                Addr::unchecked("axone1072nc6egexqr2v6vpp7yxwm68plvqnkf5uemr0"),
                &vc,
            ));

            assert!(dc_res.is_err());
            if let ContractError::UnsupportedCredential(msg) = dc_res.err().unwrap() {
                assert_eq!(msg, case.1.to_string());
            } else {
                assert!(false);
            }
        }
    }
}
