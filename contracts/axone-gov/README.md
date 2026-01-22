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
