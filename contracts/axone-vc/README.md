# AXONE Verifiable Credential Contract (`axone-vc`)

The AXONE Verifiable Credential contract attaches **verifiable credential capabilities**
to a resource represented by an **Abstract Account (AA)**.

It is the AXONE contract responsible for binding a credential authority to that resource
and structuring the credential lifecycle around submission, revocation, suspension,
reinstatement and verification.

It targets credentials represented as RDF datasets.

The authority identifier exposed by the contract currently uses the `did:pkh` method,
with the host Abstract Account rendered as a canonical Cosmos account address:

`did:pkh:cosmos:<chain_id>:cosmos1...`
