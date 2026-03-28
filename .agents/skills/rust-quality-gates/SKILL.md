---
name: rust-quality-gates
description: Repository quality gates for Rust and generated artifacts. Use when validating changes locally or before committing Rust, schema, or documentation updates.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Rust Quality Gates

## Canonical Sequence

Run the full local gate sequence with:

```bash
./.agents/skills/rust-quality-gates/scripts/run-all.sh
```

By default the script runs:

```text
format
lint
test
test-coverage
schema
docs
```

## Running a Subset

Pass explicit task names to run only a subset:

```bash
./.agents/skills/rust-quality-gates/scripts/run-all.sh lint test schema
```

## When to Run Which Gates

- Run the full sequence before finalizing substantial Rust changes.
- Always include `schema` and `docs` when message types, response types, doc comments, or `metadata.json` changed.
- Use the script output as the source of truth. Stop at the first failing gate and fix that issue before continuing.
