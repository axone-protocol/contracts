# AXONE Governance Contract (`axone-gov`)

The AXONE Governance contract attaches **governance capabilities** to a resource represented by an **Abstract Account (AA)**.

In the AXONE protocol, resources are first-class citizens. Instantiating this contract on a resource AA equips that resource with an explicit, programmable governance layer.

## Core Concepts

### Resource & Abstract Account

In AXONE, every resource is represented by an **Abstract Account (AA)**. The AA acts as the canonical on-chain identity and execution context for the resource.

The `axone-gov` contract is deployed **on top of** an AA. Doing so does not create governance in isolation: it **binds governance to the resource itself**.

### Constitution

A **constitution** is a Prolog program stored by the contract. It defines the governance rules of the resource.

Concretely, the constitution:

- Encodes governance rules as Prolog predicates
- May query on-chain facts via the AXONE logic module
- Is the single source of truth for all governance decisions

The constitution exposes the following entrypoints:

- `governance:decide/2`
- `governance:decide/3`

These predicates are validated at contract instantiation.

### Case

A **Case** represents the context submitted for a governance decision.

It is expressed as a Prolog dictionary, conventionally written as `ctx{...}`. A case may include any contextual elements required by the constitution, such as:

- intents
- actors
- subjects
- external facts or signals

On-chain facts (e.g. verifiable credentials, resource state) are **not** passed through the case. They are queried directly by the constitution itself.

## Governance Decisions

The core governance operation is the evaluation of a case through the constitution.

Depending on the query mode, the constitution returns:

- a **verdict** (`decide/2`)
- or a **verdict with motivation** (`decide/3`)

Both the verdict and the motivation are arbitrary Prolog terms. The contract does not constrain their structure.

## Constitutional Revision

The constitution is not static. It can be revised through a governance-controlled process.

A constitutional revision is performed by submitting an execution message that:

1. Proposes a new constitution
2. Evaluates the **current** constitution on a case containing the intent `gov:revise_constitution`

The revision is applied **only if** the verdict returned by the constitution is exactly:

```prolog
gov:permitted
```

Any other verdict (atom or compound term) results in a refusal.

## InstantiateMsg

Instantiate message.

In Axone, a resource is represented by an Abstract Account (AA). Instantiating this `gov` app on the resource AA attaches a **governance capability** to that resource.

A **constitution** (or governance constitution) is the Prolog program stored by this contract. It expresses the resource governance as rules that decide cases and may query on-chain facts via the Axone logic module.

The `constitution` payload MUST be a UTF-8 encoded Prolog program.

On instantiation, the contract performs a constitutive decision by asking the constitution to decide the intent `gov:establish` using a contract-enriched case (including `gov:module` metadata and the `cw:tx` runtime context). The app is instantiated only if the verdict is exactly the atom `gov:permitted`.

Precondition: the `constitution` payload MUST be a UTF-8 encoded Prolog program.

The contract validates that the program can be evaluated and that it defines the required entrypoints:

- `decide/2` as `governance:decide(+Case, -Verdict)`

- `decide/3` as `governance:decide(+Case, -Verdict, -Motivation)`

Where:

- `Case` is a Prolog dict term (typically `ctx{...}`) representing the decision context. It can include any key-value facts required by the constitution (e.g. intent, actor, subject).

- `Verdict` is an arbitrary Prolog term (atom or compound) representing the decision outcome.

- `Motivation` is an arbitrary Prolog term intended to justify the verdict (e.g. applicable articles, findings, interpretation rules).

The case structure used for `gov:establish` follows the same conventions as other governance acts (see `ReviseConstitution`), including `gov:module` metadata and the `cw:tx` runtime context.

| variant        | description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| InstantiateMsg | **object**. Instantiate message.<br /><br />In Axone, a resource is represented by an Abstract Account (AA). Instantiating this `gov` app on the resource AA attaches a **governance capability** to that resource.<br /><br />A **constitution** (or governance constitution) is the Prolog program stored by this contract. It expresses the resource governance as rules that decide cases and may query on-chain facts via the Axone logic module.<br /><br />The `constitution` payload MUST be a UTF-8 encoded Prolog program.<br /><br />On instantiation, the contract performs a constitutive decision by asking the constitution to decide the intent `gov:establish` using a contract-enriched case (including `gov:module` metadata and the `cw:tx` runtime context). The app is instantiated only if the verdict is exactly the atom `gov:permitted`.<br /><br />Precondition: the `constitution` payload MUST be a UTF-8 encoded Prolog program.<br /><br />The contract validates that the program can be evaluated and that it defines the required entrypoints:<br /><br />- `decide/2` as `governance:decide(+Case, -Verdict)`<br /><br />- `decide/3` as `governance:decide(+Case, -Verdict, -Motivation)`<br /><br />Where:<br /><br />- `Case` is a Prolog dict term (typically `ctx{...}`) representing the decision context. It can include any key-value facts required by the constitution (e.g. intent, actor, subject).<br /><br />- `Verdict` is an arbitrary Prolog term (atom or compound) representing the decision outcome.<br /><br />- `Motivation` is an arbitrary Prolog term intended to justify the verdict (e.g. applicable articles, findings, interpretation rules).<br /><br />The case structure used for `gov:establish` follows the same conventions as other governance acts (see `ReviseConstitution`), including `gov:module` metadata and the `cw:tx` runtime context. |

## ExecuteMsg

Execute messages.

### ExecuteMsg::record_decision

Record a decision on-chain by deciding a case using the stored constitution.

The `case` parameter is a Prolog dict term string (typically `ctx{...}`) representing the decision context provided by the caller.

Before evaluation, the contract enriches the case with contract-derived facts:

`prolog ctx{ 'gov:module': module{ id: &lt;atom&gt;, version: &lt;atom&gt; }, 'cw:tx': tx{ message: msg{ sender: &lt;atom&gt;,                    % Bech32 address of message sender funds: [coin(Amount, Denom), ...]  % List of coins sent with message }, block: block{ height: &lt;integer&gt;,        % Block height time_seconds: &lt;integer&gt;,  % Block timestamp (seconds since epoch) tx_index: &lt;integer&gt;       % Transaction index (optional) } }, &lt;caller_provided_keys&gt;: &lt;caller_provided_values&gt; } `

Injected keys are authoritative and overwrite any caller-provided value under the same keys.

The contract evaluates `governance:decide/2` or `governance:decide/3` depending on `motivated`, and records the resulting verdict (and optional motivation) as a durable decision record.

| parameter                   | description                                                                                                                                                                                                                                                                      |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `record_decision`           | _(Required.) _ **object**.                                                                                                                                                                                                                                                       |
| `record_decision.case`      | _(Required.) _ **string**. The decision context.                                                                                                                                                                                                                                 |
| `record_decision.motivated` | **boolean\|null**. Whether to request a motivated decision (defaults to `false`).<br /><br />- If `false`, the contract calls `governance:decide/2` and records only the verdict. - If `true`, the contract calls `governance:decide/3` and records both verdict and motivation. |

### ExecuteMsg::revise_constitution

Propose a constitutional revision (constitutional amendment).

The revision is a two-step governance act:

1. The contract asks the **current** constitution to decide whether the revision is allowed by evaluating a case that includes the intent `gov:revise_constitution`.

2. The contract then asks the **proposed** constitution to decide the constitutive intent `gov:establish`, using the same contract-enriched case (including `cw:tx` funds).

The complete case structure is (keys containing `:` are quoted atoms):

`prolog ctx{ intent: &lt;intent_atom&gt;, % 'gov:revise_constitution' or 'gov:establish' 'gov:proposed_constitution_sha256': &lt;hex_atom&gt;, 'gov:current_constitution_sha256': &lt;hex_atom&gt;, 'gov:current_constitution_revision': &lt;integer&gt;, 'gov:module': module{ id: &lt;atom&gt;,       % Contract module ID (e.g., 'axone:axone-gov') version: &lt;atom&gt;   % Contract version (e.g., '1.2.3') }, 'cw:tx': tx{ message: msg{ sender: &lt;atom&gt;,                    % Bech32 address of message sender funds: [coin(Amount, Denom), ...]  % List of coins sent with message }, block: block{ height: &lt;integer&gt;,        % Block height time_seconds: &lt;integer&gt;,  % Block timestamp (seconds since epoch) tx_index: &lt;integer&gt;       % Transaction index } }, &lt;caller_provided_keys&gt;: &lt;caller_provided_values&gt;  % Any additional keys from caller's case } `

The revision is applied only if both decisions return the verdict `gov:permitted`.

| parameter                          | description                                                                                                                                                                                                                    |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `revise_constitution`              | _(Required.) _ **object**.                                                                                                                                                                                                     |
| `revise_constitution.case`         | **string\|null**. Optional additional decision context provided by the caller.<br /><br />This is a Prolog dict term string (typically `ctx{...}`) merged into the case used to evaluate the `gov:revise_constitution` intent. |
| `revise_constitution.constitution` | _(Required.) _ **[Binary](#binary)**. The proposed new constitution (UTF-8 Prolog program bytes).                                                                                                                              |

## QueryMsg

Query messages.

### QueryMsg::constitution

Return the currently stored constitution (raw Prolog program bytes).

| parameter      | description                |
| -------------- | -------------------------- |
| `constitution` | _(Required.) _ **object**. |

### QueryMsg::constitution_status

Return the current constitution metadata (revision and hash).

| parameter             | description                |
| --------------------- | -------------------------- |
| `constitution_status` | _(Required.) _ **object**. |

### QueryMsg::decide

Decide a case using the stored constitution.

The `case` parameter is a Prolog dict term string (typically `ctx{...}`) representing the decision context. This is passed as the `Case` argument to `governance:decide/2` or `governance:decide/3`.

Example:

`ctx{intent:read, user:"did:example:123", object:"obj:42"}`

The returned `verdict` is an arbitrary Prolog term (atom or compound), for example:

- `gov:permitted`

- `gov:forbidden`

- `pay("did:...", 1000)`

The optional `motivation` is an arbitrary Prolog term returned by the constitution and intended to justify the verdict (e.g. grounds/articles, findings, interpretation rules).

Before evaluation, the contract enriches the case with module metadata (`'gov:module'`).

Injected keys are authoritative and overwrite any caller-provided value under the same keys.

| parameter          | description                                                                                                                                                                                                                                                                      |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decide`           | _(Required.) _ **object**.                                                                                                                                                                                                                                                       |
| `decide.case`      | _(Required.) _ **string**. The decision context.                                                                                                                                                                                                                                 |
| `decide.motivated` | **boolean\|null**. Whether to request a motivated decision (defaults to `false`).<br /><br />- If `false`, the contract calls `governance:decide/2` and returns only the verdict. - If `true`, the contract calls `governance:decide/3` and returns both verdict and motivation. |

### QueryMsg::decision

Return a recorded decision by its unique identifier.

The returned record is created by `ExecuteMsg::RecordDecision` and includes the decision payload (case/verdict, optional motivation) along with constitution metadata (revision/hash) and block metadata.

| parameter              | description                                                 |
| ---------------------- | ----------------------------------------------------------- |
| `decision`             | _(Required.) _ **object**.                                  |
| `decision.decision_id` | _(Required.) _ **integer**. The unique decision identifier. |

### QueryMsg::decisions

Return a paginated list of recorded decisions.

Decisions are ordered by their unique identifier in ascending order.

| parameter               | description                                                                      |
| ----------------------- | -------------------------------------------------------------------------------- |
| `decisions`             | _(Required.) _ **object**.                                                       |
| `decisions.limit`       | **integer\|null**. Optional maximum number of decisions to return (default: 10). |
| `decisions.start_after` | **integer\|null**. Optional decision ID to start after (exclusive).              |

## MigrateMsg

Migrate message.

Reserved for future migrations.

### MigrateMsg::MigrateMsg

Migrate message.

Reserved for future migrations.

| parameter | description |
| --------- | ----------- |

## Responses

### constitution

Response returned by `QueryMsg::Constitution`.

| property       | description                                                                               |
| -------------- | ----------------------------------------------------------------------------------------- |
| `constitution` | _(Required.) _ **[Binary](#binary)**. The stored constitution (raw Prolog program bytes). |

### constitution_status

Response returned by `QueryMsg::ConstitutionStatus`.

| property                | description                                                                                                                                                                       |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `constitution_hash`     | _(Required.) _ **[Binary](#binary)**. The stored constitution hash (32 bytes, sha256).                                                                                            |
| `constitution_revision` | _(Required.) _ **integer**. The constitution revision number.<br /><br />The initially instantiated constitution has revision `0`. Each successful revision increments it by `1`. |

### decide

Response returned by `QueryMsg::Decide`.

| property     | description                                                                                  |
| ------------ | -------------------------------------------------------------------------------------------- |
| `motivation` | **string\|null**. Optional motivation term returned as the third argument of `decide/3`.     |
| `verdict`    | _(Required.) _ **string**. The verdict returned by the constitution as a Prolog term string. |

### decision

Response returned by `QueryMsg::Decision`.

| property                | description                                                                                             |
| ----------------------- | ------------------------------------------------------------------------------------------------------- |
| `author`                | _(Required.) _ **string**. The author Bech32 address.                                                   |
| `block_height`          | _(Required.) _ **integer**. The block height at which the decision was recorded.                        |
| `block_time_seconds`    | _(Required.) _ **integer**. The block time (seconds since epoch) at which the decision was recorded.    |
| `case`                  | _(Required.) _ **string**. The case term as a Prolog term string.                                       |
| `case_hash`             | _(Required.) _ **[Binary](#binary)**. The case hash (32 bytes, sha256).                                 |
| `constitution_hash`     | _(Required.) _ **[Binary](#binary)**. The constitution hash at the time of decision (32 bytes, sha256). |
| `constitution_revision` | _(Required.) _ **integer**. The constitution revision number at the time of decision.                   |
| `decision_id`           | _(Required.) _ **integer**. The unique decision identifier.                                             |
| `motivation`            | **string\|null**. Optional motivation term as a Prolog term string.                                     |
| `motivation_hash`       | **[Binary](#binary)\|null**. The motivation hash (32 bytes, sha256).                                    |
| `verdict`               | _(Required.) _ **string**. The verdict term as a Prolog term string.                                    |
| `verdict_hash`          | _(Required.) _ **[Binary](#binary)**. The verdict hash (32 bytes, sha256).                              |

### decisions

Response returned by `QueryMsg::Decisions`.

| property    | description                                                            |
| ----------- | ---------------------------------------------------------------------- |
| `decisions` | _(Required.) _ **Array&lt;[DecisionResponse](#decisionresponse)&gt;**. |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### DecisionResponse

Response returned by `QueryMsg::Decision`.

| property                | description                                                                                             |
| ----------------------- | ------------------------------------------------------------------------------------------------------- |
| `author`                | _(Required.) _ **string**. The author Bech32 address.                                                   |
| `block_height`          | _(Required.) _ **integer**. The block height at which the decision was recorded.                        |
| `block_time_seconds`    | _(Required.) _ **integer**. The block time (seconds since epoch) at which the decision was recorded.    |
| `case`                  | _(Required.) _ **string**. The case term as a Prolog term string.                                       |
| `case_hash`             | _(Required.) _ **[Binary](#binary)**. The case hash (32 bytes, sha256).                                 |
| `constitution_hash`     | _(Required.) _ **[Binary](#binary)**. The constitution hash at the time of decision (32 bytes, sha256). |
| `constitution_revision` | _(Required.) _ **integer**. The constitution revision number at the time of decision.                   |
| `decision_id`           | _(Required.) _ **integer**. The unique decision identifier.                                             |
| `motivation`            | **string\|null**. Optional motivation term as a Prolog term string.                                     |
| `motivation_hash`       | **[Binary](#binary)\|null**. The motivation hash (32 bytes, sha256).                                    |
| `verdict`               | _(Required.) _ **string**. The verdict term as a Prolog term string.                                    |
| `verdict_hash`          | _(Required.) _ **[Binary](#binary)**. The verdict hash (32 bytes, sha256).                              |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-gov.json` (`12c459f623aed8e1`)_
