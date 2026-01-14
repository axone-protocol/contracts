# Conventional Commit Examples

## Features

```text
feat(gov): introduce quadratic voting mechanism
feat(handlers): implement proposal execution flow
feat(state): support multi-sig ownership
```

## Bug Fixes

```text
fix(handlers): prevent overflow in vote counting
fix(query): handle empty pagination correctly
fix(state): validate storage key uniqueness
```

## Refactoring

```text
refactor(state): clarify storage key naming
refactor(handlers): extract common validation logic
refactor(msg): align response types with API spec
```

## Tests

```text
test(gov): validate error paths for unauthorized access
test(handlers): ensure edge cases in voting
test(query): verify pagination boundaries
```

## Build & Dependencies

```text
build(deps): enforce abstract-sdk 0.26.1
build(make): tighten wasm optimization flags
```

## CI

```text
ci(workflow): tighten linting rules
ci(dependabot): align update schedule
```

## Documentation

```text
docs(README): document deployment workflow
docs(gov): clarify instantiation parameters
```

## Performance

```text
perf(query): optimize pagination iterator
perf(state): reduce storage reads in vote tally
```

## Breaking Changes

```text
feat(msg)!: restructure ExecuteMsg variants
refactor(api)!: enforce stricter validation schema
```

## Bad Examples ❌

| Bad | Why |
| - | - |
| `feat(gov): add voting` | Weak verb, vague |
| `fix: fix bug` | Meaningless |
| `refactor: refactor code` | Says nothing |
| `feat(axone-gov): add new feature` | Redundant prefix, weak verb |
| `docs: update docs` | Generic |
| `feat(gov): add voting mechanism that allows users to...` | Too long |
