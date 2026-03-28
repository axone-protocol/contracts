# AXONE Verifiable Credential Contract (`axone-vc`)

The AXONE Verifiable Credential contract attaches **verifiable credential capabilities**
to a resource represented by an **Abstract Account (AA)**.

It is the AXONE contract responsible for binding a credential authority to that resource
and structuring the credential lifecycle around submission, revocation, suspension,
reinstatement and verification.

It targets credentials represented as RDF datasets.

## Authority

The contract exposes the identifier of the credential authority through the `Authority`
query.

This identifier is the DID of the resource bound to the host Abstract Account for this
VC capability.

The current representation uses the `did:pkh` method and is grounded in the on-chain
address of the host Abstract Account, rendered as a CAIP-compatible canonical Cosmos
Bech32 account address.

Form:

`did:pkh:cosmos:<chain_id>:cosmos1...`
