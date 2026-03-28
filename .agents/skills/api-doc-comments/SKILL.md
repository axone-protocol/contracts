---
name: api-doc-comments
description: Guide for writing Rust doc comments that produce accurate generated contract documentation. Use when editing Instantiate/Execute/Query/Response types or any public schema-facing API.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# API Doc Comments

This skill covers how to write schema-friendly Rust doc comments for Axone smart contract APIs.

## Core Rule

The generated documentation is only as good as the Rust doc comments on schema-facing types.

Write the comments on the source types first, then regenerate docs with the doc-generation workflow.

## What must be documented

Document all schema-facing public items:

- message structs and enums
- execute/query variants
- every public field in those messages
- query response structs
- domain-specific public enums surfaced through the schema

## Writing Style

Prefer comments that are:

- specific to the contract's domain
- precise about behavior
- concise, but not cryptic
- written from the caller's point of view

Avoid comments that are:

- generic restatements of the field name
- implementation-oriented when the caller needs semantics
- padded with filler text

## What good API comments should explain

### Semantics

Explain what the message or field means in the protocol, not only its Rust type.

### Preconditions and invariants

Document constraints such as:

- accepted formats
- authorization requirements
- default behaviors
- ordering or pagination semantics
- exact conditions under which an action is permitted

### Encoded representations

When the public API intentionally uses `String` or `Binary`, document the encoded format explicitly.

Examples from this repository include:

- Prolog case terms carried as `String`
- constitutions carried as UTF-8 Prolog bytes in `Binary`
- DIDs represented as canonical strings
- hashes exposed as binary values with a named algorithm

### Domain examples

Use short examples when the payload format is not self-evident, especially for:

- Prolog terms
- CAIP / DID-like identifiers
- structured intent names

## Repo-Specific Guidance

For Axone contracts, strong API comments often need to explain:

- how a resource relates to its host Abstract Account
- what data is caller-provided versus injected by the contract
- which keys are authoritative when contexts are merged
- which governance intent is being evaluated
- what a returned verdict or motivation represents

These semantics matter more than low-level implementation detail.

## Recommended Comment Shapes

### Top-level message types

- one short summary line
- one or more paragraphs for domain semantics
- preconditions or protocol rules when relevant

### Enum variants

- what the action or query does
- when it should be used
- important side effects or gating rules

### Fields

- what the field contains
- expected format or units
- default or optional behavior if applicable

## Examples of useful details

Good details to include:

- "UTF-8 Prolog program bytes"
- "exclusive pagination cursor"
- "verdict returned by `governance:decide/3`"
- "canonical Cosmos Bech32 account rendered in DID form"

Weak details to avoid:

- "The title"
- "A string value"
- "Used for execution"

## Relationship with other skills

- Use `api-design` to shape the contract surface itself.
- Use this skill to make that surface intelligible in generated documentation.
- Use `doc-generation` after comment changes to refresh generated artifacts.

## Quick Checklist

- [ ] Every public schema-facing type has a meaningful doc comment
- [ ] Every public field is documented
- [ ] Constraints and defaults are stated explicitly
- [ ] Domain-specific encodings are explained
- [ ] Examples are included when the payload syntax is non-obvious
- [ ] Comments describe caller-visible behavior, not internal mechanics
