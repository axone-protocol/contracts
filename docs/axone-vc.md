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

## InstantiateMsg

Instantiate message.

Instantiating this app attaches a verifiable credential authority to the resource represented by the host Abstract Account.

This contract requires no caller-provided configuration.

| variant        | description                                                                                                                                                                                                                                 |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| InstantiateMsg | **object**. Instantiate message.<br /><br />Instantiating this app attaches a verifiable credential authority to the resource represented by the host Abstract Account.<br /><br />This contract requires no caller-provided configuration. |

## ExecuteMsg

Execute messages.

### ExecuteMsg::issue_credential

Issue a verifiable credential from this authority.

Only the app authority is allowed to call this message.

The submitted payload must match the declared `format` and must describe exactly one credential that satisfies the contract invariants.

The credential is accepted only if it provides: - an identifier - either no issuer or an issuer equal to the authority DID exposed by this contract - a subject identifier - at least one type, including `VerifiableCredential` - optional `validFrom` and `validUntil` claims, when present, encoded as `xsd:dateTimeStamp` instants with `validFrom &lt; validUntil`

When the submitted payload omits the issuer, the contract adds its authority DID as the credential issuer.

The registered credential therefore always contains the effective issuer.

Issuance fails if the payload format is not supported, if the credential representation cannot be interpreted according to that format, or if a credential with the same identifier has already been issued by this authority.

| parameter                     | description                                                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `issue_credential`            | _(Required.) _ **object**.                                                                                                                                                     |
| `issue_credential.credential` | _(Required.) _ **[Binary](#binary)**. Serialized credential payload.<br /><br />The expected binary encoding and semantic representation are determined by the `format` field. |
| `issue_credential.format`     | **[CredentialInputFormat](#credentialinputformat)\|null**. Encoding used by the submitted credential payload.<br /><br />Defaults to `n_quads` when omitted.                   |

### ExecuteMsg::revoke_credential

Revoke a verifiable credential from this authority.

Only the app authority is allowed to call this message.

The revocation is terminal, the same identifier cannot be issued again.

Revocation fails if the identifier is unknown or already revoked.

| parameter                      | description                                                     |
| ------------------------------ | --------------------------------------------------------------- |
| `revoke_credential`            | _(Required.) _ **object**.                                      |
| `revoke_credential.identifier` | _(Required.) _ **string**. The credential identifier to revoke. |

## QueryMsg

Query messages.

### QueryMsg::authority

Return the DID of the credential authority attached to this contract.

This identifier is the authority identity recognized by the contract for issuing and managing credentials on behalf of the attached resource.

The returned DID uses the `did:pkh` method and is grounded in the on-chain address of the host Abstract Account, rendered as a CAIP-compatible canonical Cosmos Bech32 account address.

Form:

`did:pkh:cosmos:&lt;chain_id&gt;:cosmos1...`

| parameter   | description                |
| ----------- | -------------------------- |
| `authority` | _(Required.) _ **object**. |

### QueryMsg::verify_credential

Check whether a credential is active and, optionally, valid at a given instant.

Revoked credentials are not part of the active credential set and therefore return the same result as unknown identifiers.

| parameter                      | description                                                                                                                                                                      |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `verify_credential`            | _(Required.) _ **object**.                                                                                                                                                       |
| `verify_credential.identifier` | _(Required.) _ **string**. Identifier of the credential to check.                                                                                                                |
| `verify_credential.valid_at`   | **[Timestamp](#timestamp)\|null**. Optional instant at which to evaluate the credential validity interval.<br /><br />When omitted, every active credential is considered valid. |

### QueryMsg::credential_raw

Return the canonical serialized representation stored for an active credential.

The returned representation is the contract's canonical storage format. It is not guaranteed to preserve the encoding or presentation of the issued payload.

This query fails when the identifier is unknown or the credential has been revoked.

| parameter                   | description                                                          |
| --------------------------- | -------------------------------------------------------------------- |
| `credential_raw`            | _(Required.) _ **object**.                                           |
| `credential_raw.identifier` | _(Required.) _ **string**. Identifier of the credential to retrieve. |

### QueryMsg::credential

Return an active issued credential with its RDF dataset.

The returned metadata is reconstructed from the credential RDF dataset accepted at issuance. The `quads` field contains the canonical dataset as structured RDF quads.

This query fails when the identifier is unknown or the credential has been revoked.

| parameter               | description                                                          |
| ----------------------- | -------------------------------------------------------------------- |
| `credential`            | _(Required.) _ **object**.                                           |
| `credential.identifier` | _(Required.) _ **string**. Identifier of the credential to retrieve. |

## MigrateMsg

Migrate message.

Reserved for future migrations.

### MigrateMsg::MigrateMsg

Migrate message.

Reserved for future migrations.

| parameter | description |
| --------- | ----------- |

## Responses

### authority

Response returned by `AxoneVcQueryMsg::Authority`.

| property | description                                                                                                                                                                                                                                                                                                                                |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `did`    | _(Required.) _ **string**. The authority DID recognized by this contract.<br /><br />This representation uses the `did:pkh` method over the on-chain address of the host Abstract Account, rendered as a CAIP-compatible canonical Cosmos Bech32 account address.<br /><br />Form:<br /><br />`did:pkh:cosmos:&lt;chain_id&gt;:cosmos1...` |

### credential

Response returned by `AxoneVcQueryMsg::Credential`.

| property      | description                                                                                                      |
| ------------- | ---------------------------------------------------------------------------------------------------------------- |
| `identifier`  | _(Required.) _ **string**. Credential identifier extracted from the VC `id`.                                     |
| `issuer`      | _(Required.) _ **string**. Authority DID recorded as the credential issuer.                                      |
| `quads`       | _(Required.) _ **Array&lt;[Quad](#quad)&gt;**. Canonical credential RDF dataset represented as structured quads. |
| `subject`     | _(Required.) _ **string**. Credential subject identifier.                                                        |
| `types`       | _(Required.) _ **Array&lt;string&gt;**. Credential type URIs extracted from the VC `type` values.                |
| `valid_from`  | **[Timestamp](#timestamp)\|null**. Optional lower bound of the credential validity interval.                     |
| `valid_until` | **[Timestamp](#timestamp)\|null**. Optional exclusive upper bound of the credential validity interval.           |

### credential_raw

Response returned by `AxoneVcQueryMsg::CredentialRaw`.

| property     | description                                                                                                                                                                                                                                                                   |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `credential` | _(Required.) _ **[Binary](#binary)**. Canonical serialized credential representation persisted by the contract.<br /><br />This binary value is base64-encoded in JSON responses and is independent from the format and presentation of the credential submitted at issuance. |

### verify_credential

Response returned by `AxoneVcQueryMsg::VerifyCredential`.

| property | description                                                                                                                                                       |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `exists` | _(Required.) _ **boolean**. Whether the identifier belongs to an active credential.                                                                               |
| `valid`  | _(Required.) _ **boolean**. Whether the active credential is valid for the requested instant.<br /><br />This is equal to `exists` when no instant was requested. |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### CredentialInputFormat

Supported credential input encodings.

| variant   | description                                                                                                                                                                                                                                                      |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| undefined | **string**: `n_quads`. UTF-8 RDF dataset serialized as N-Quads.<br /><br />N-Quads extends N-Triples to represent RDF datasets by allowing an optional fourth term that carries the graph name. See the [N-Quads specification](https://www.w3.org/TR/n-quads/). |

### Quad

RDF quad returned by `AxoneVcQueryMsg::Credential`.

| property     | description                                                                                            |
| ------------ | ------------------------------------------------------------------------------------------------------ |
| `graph_name` | **string\|null**. RDF graph name serialized with N-Quads term syntax, or `None` for the default graph. |
| `object`     | _(Required.) _ **string**. RDF object term serialized with N-Quads term syntax.                        |
| `predicate`  | _(Required.) _ **string**. RDF predicate IRI serialized with N-Quads term syntax.                      |
| `subject`    | _(Required.) _ **string**. RDF subject term serialized with N-Quads term syntax.                       |

### Timestamp

A point in time in nanosecond precision.

This type can represent times from 1970-01-01T00:00:00Z to 2554-07-21T23:34:33Z.

## Examples

````# use cosmwasm_std::Timestamp; let ts = Timestamp::from_nanos(1_000_000_202); assert_eq!(ts.nanos(), 1_000_000_202); assert_eq!(ts.seconds(), 1); assert_eq!(ts.subsec_nanos(), 202);

let ts = ts.plus_seconds(2); assert_eq!(ts.nanos(), 3_000_000_202); assert_eq!(ts.seconds(), 3); assert_eq!(ts.subsec_nanos(), 202); ```



### Uint64

A thin wrapper around u64 that is using strings for JSON encoding/decoding, such that the full u64 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u64` to get the value out:

``` # use cosmwasm_std::Uint64; let a = Uint64::from(42u64); assert_eq!(a.u64(), 42);

let b = Uint64::from(70u32); assert_eq!(b.u64(), 70); ```

|type|
|----|
|**string**.|

### undefined

UTF-8 RDF dataset serialized as N-Quads.

N-Quads extends N-Triples to represent RDF datasets by allowing an optional fourth term that carries the graph name. See the [N-Quads specification](https://www.w3.org/TR/n-quads/).

|literal|
|-------|
|`"n_quads"`|

---

*Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-vc.json` (`b0e41424a881dbde`)*
````
