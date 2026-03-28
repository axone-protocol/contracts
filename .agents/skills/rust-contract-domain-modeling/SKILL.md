---
name: rust-contract-domain-modeling
description: Domain-driven modeling patterns for Axone contracts. Use when introducing domain concepts, encoding invariants, or deciding boundaries between domain, handlers, services, gateways, queries, and state.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Rust Contract Domain Modeling

## Goal

Keep business invariants in explicit domain types instead of scattering them across handlers, query builders, or storage code.

## Boundary Rules

- `domain/` owns business concepts and invariants.
- `handlers/` decode messages, orchestrate use cases, and shape responses.
- `services/` compose domain logic with environment-dependent enrichment or cross-module coordination.
- `gateway/` isolates external module interaction and protocol-specific I/O.
- `queries/` build external query payloads or request strings; they should not become the home of business rules.
- `state/` persists data and reconstructs domain values; it should not silently redefine domain invariants.

## Domain Rules

- Prefer constructors such as `new` or `try_new` that reject invalid states up front.
- Use `TryFrom` when validating a parsed or transport-level representation into a domain type.
- Keep domain methods deterministic and side-effect-light whenever possible.
- If a value is reconstructed from trusted state, make that explicit with a constructor such as `from_state`.
- Model canonical representations once in the domain layer, then reuse them everywhere else.

## Patterns From This Repository

- `Case::new` and `TryFrom<Term>` validate that a case is a ground Prolog dict before it can circulate as a domain object.
- `Constitution::try_new` validates UTF-8, required predicates, and engine feedback before a constitution exists as a valid domain value.
- `Constitution::from_state` reconstructs a previously validated value from storage without re-running external validation.
- `Decision::new` captures an immutable decision snapshot from already validated inputs.
- `Authority::new` canonicalizes a bech32 account into the DID form exposed by the contract.

## Design Heuristics

- Put parsing and invariant checks as close as possible to the creation of the domain object.
- Keep handlers thin. If a handler starts accumulating validation branches, move that logic into a domain type or service.
- Avoid passing partially validated strings through multiple layers when a dedicated type can encode the guarantee once.
- Make invalid states unrepresentable where practical, especially for smart-contract-critical logic.

## References

- For concrete domain examples from this repository, read [repo-patterns](./references/repo-patterns.md).
