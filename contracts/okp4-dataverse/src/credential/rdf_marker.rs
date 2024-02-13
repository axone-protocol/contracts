use rio_api::model::{NamedNode, Term};

pub const RDF_TYPE: NamedNode<'_> = NamedNode {
    iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
};
pub const RDF_CREATED: NamedNode<'_> = NamedNode {
    iri: "http://purl.org/dc/terms/created",
};
pub const RDF_DATE_TYPE: NamedNode<'_> = NamedNode {
    iri: "http://www.w3.org/2001/XMLSchema#dateTime",
};

pub const IRI_VC_TYPE: &str = "https://www.w3.org/2018/credentials#VerifiableCredential";
pub const VC_RDF_TYPE: Term<'_> = Term::NamedNode(NamedNode { iri: IRI_VC_TYPE });
pub const VC_RDF_ISSUER: NamedNode<'_> = NamedNode {
    iri: "https://www.w3.org/2018/credentials#issuer",
};
pub const VC_RDF_ISSUANCE_DATE: NamedNode<'_> = NamedNode {
    iri: "https://www.w3.org/2018/credentials#issuanceDate",
};
pub const VC_RDF_EXPIRATION_DATE: NamedNode<'_> = NamedNode {
    iri: "https://www.w3.org/2018/credentials#expirationDate",
};
pub const VC_RDF_CREDENTIAL_SUBJECT: NamedNode<'_> = NamedNode {
    iri: "https://www.w3.org/2018/credentials#credentialSubject",
};
pub const VC_RDF_CREDENTIAL_STATUS: NamedNode<'_> = NamedNode {
    iri: "https://www.w3.org/2018/credentials#credentialStatus",
};

pub const VC_RDF_PROOF: NamedNode<'_> = NamedNode {
    iri: "https://w3id.org/security#proof",
};
pub const PROOF_RDF_VERIFICATION_METHOD: NamedNode<'_> = NamedNode {
    iri: "https://w3id.org/security#verificationMethod",
};
pub const PROOF_RDF_PROOF_PURPOSE: NamedNode<'_> = NamedNode {
    iri: "https://w3id.org/security#proofPurpose",
};
pub const PROOF_RDF_PROOF_VALUE: NamedNode<'_> = NamedNode {
    iri: "https://w3id.org/security#proofValue",
};
pub const PROOF_RDF_JWS: NamedNode<'_> = NamedNode {
    iri: "https://w3id.org/security#jws",
};
pub const PROOF_RDF_PROOF_VALUE_TYPE: NamedNode<'_> = NamedNode {
    iri: "https://w3id.org/security#multibase",
};
pub const PROOF_RDF_CRYPTOSUITE: NamedNode<'_> = NamedNode {
    iri: "https://w3id.org/security#cryptosuite",
};
