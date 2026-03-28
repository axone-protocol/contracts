# Repository Domain Patterns

These examples show how the repository currently uses explicit domain types to encode invariants and boundaries.

## Case

`contracts/axone-gov/src/domain/case.rs`

- `Case::new` parses a string into a `Term`, then validates that the result is a ground Prolog dict.
- `TryFrom<Term>` keeps the invariant at the boundary between parsed syntax and domain value.
- `merge` preserves the case abstraction instead of leaking raw term manipulation into handlers.

## Constitution

`contracts/axone-gov/src/domain/constitution.rs`

- `Constitution::try_new` validates UTF-8, predicate presence, and logic-engine feedback before constructing the domain object.
- `Constitution::from_state` makes the trusted reconstruction path explicit.
- Hashing and canonical source access live on the domain object, not in callers.

## Decision

`contracts/axone-gov/src/domain/decision.rs`

- `Decision::new` builds a stable snapshot from validated inputs and current constitution status.
- The type captures domain facts such as verdict, motivation, author, and block metadata in one place.

## Authority

`contracts/axone-vc/src/domain/authority.rs`

- `Authority::new` canonicalizes addresses before exposing the DID string.
- The canonicalization rule exists once, in the domain object, instead of being repeated in handlers or queries.

## Handler / Service Split

`contracts/axone-gov/src/handlers/instantiate.rs`

- The handler orchestrates the use case, but domain validation is delegated to `Constitution::try_new`.
- Governance-case enrichment and decision evaluation are delegated to `services/decision.rs`.

`contracts/axone-gov/src/services/decision.rs`

- The service builds enriched cases and coordinates external logic queries.
- Domain objects such as `Case`, `Constitution`, and `ConstitutionStatus` remain the stable types passed across layers.
