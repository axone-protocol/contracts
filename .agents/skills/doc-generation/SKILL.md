---
name: doc-generation
description: Guide for regenerating Axone contract schemas and rendered Markdown docs. Use when contract APIs or metadata change, when checking generated-doc drift, or when preparing documentation commits.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Generated Documentation

## Source of Truth

Generated docs come from Rust API types and schema metadata:

```text
Rust messages/types + metadata.json
              ↓
      contracts/*/schema/*
              ↓
          docs/*.md
```

In this repository, the canonical command is:

```bash
cargo make docs
```

Do not treat `docs/*.md` as hand-edited source files. The Rust types and metadata are the source of truth.

## What `cargo make docs` really does

`cargo make docs` already depends on:

- prerequisite checks (`npx`, `awk`, `perl`, `jq`)
- `cargo make schema`

That means one docs refresh can update both:

- `contracts/*/schema/*`
- `docs/*.md`

## Standard Workflow

### Regenerate everything

```bash
cargo make docs
```

### Inspect what changed

```bash
git status --short
git diff -- docs contracts
```

### Commit the generated artifacts

If the change is documentation generation only, prefer a message such as:

```text
docs(gov): regenerate documentation
docs(vc): regenerate documentation
docs: regenerate generated documentation
```

Avoid vague subjects such as `docs: update generated documentation`.

## When regeneration is required

Refresh generated docs whenever you change:

- message types in `msg.rs`
- response types exported in schemas
- doc comments that feed schema descriptions
- `metadata.json`
- schema generation code in `src/bin/schema.rs`
- the docs generation pipeline in `Makefile.toml`

## File Expectations

After regeneration, review and commit all relevant generated artifacts:

- `docs/*.md`
- `contracts/*/schema/*`

Even if CI only reports drift on `docs/*.md`, schema files are still generated source artifacts in this repo and should stay in sync with the code.

## Repo-Specific Notes

- `cargo make docs` is the preferred entrypoint; it already triggers schema generation.
- The docs renderer uses `@fadroma/schema`, `jq`, `awk`, `perl`, and `prettier` through `Makefile.toml`.
- The generated docs reflect the semantics encoded in Rust doc comments. Fix the Rust comments first, then regenerate.
