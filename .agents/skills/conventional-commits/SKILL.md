---
name: conventional-commits
description: Guide for writing conventional commit messages. Use when committing changes, writing commit messages, or reviewing commit history.
license: BSD-3-Clause
metadata:
  author: axone.xyz
  version: "1.0"
---

# Conventional Commits

This skill helps you write commit messages following the [Conventional Commits](https://www.conventionalcommits.org/) specification, adapted to Axone protocol preferences.

## When to use this skill

Use this skill when you need to:

- Write commit messages for changes
- Review or improve commit messages
- Structure commits for a pull request

## Commit Message Format

```text
<type>(<scope>): <verb> <subject>
```

One line only. No body. No footer (except for breaking changes).

## Subject Line Rules

- **Imperative mood, present tense**
- **Short, dense, unambiguous**
- **Describes the intent**, not the implementation
- **One line only**
- No capitalization at start
- No period at end

## Verb Selection

> ⚠️ **Avoid weak verbs**: `add`, `remove`, `change`, `update`, `modify`

Use **precise, action-oriented verbs**:

| Verb | Use When |
| - | - |
| `enforce` | Adding constraints or rules |
| `introduce` | Bringing in new concepts/APIs |
| `implement` | Building out functionality |
| `prevent` | Blocking undesired behavior |
| `fix` | Correcting bugs |
| `refactor` | Restructuring without behavior change |
| `clarify` | Improving readability/naming |
| `align` | Making consistent with standards |
| `tighten` | Strengthening validation/constraints |
| `harden` | Security or robustness improvements |
| `validate` | Input/state verification |
| `handle` | Managing edge cases |
| `support` | Enabling new use cases |
| `ensure` | Guaranteeing invariants |
| `document` | Documentation work |

## Type

| Type | Description | Triggers |
| - | - | - |
| `feat` | New feature | Minor version bump |
| `fix` | Bug fix | Patch version bump |
| `docs` | Documentation only | No release |
| `style` | Formatting, whitespace | No release |
| `refactor` | Code restructuring | No release |
| `perf` | Performance improvement | Patch version bump |
| `test` | Adding/updating tests | No release |
| `build` | Build system, dependencies | No release |
| `ci` | CI/CD configuration | No release |
| `chore` | Maintenance tasks | No release |

## Scope

- **Mandatory when it adds clarity**
- Short, meaningful, domain or component oriented
- Use contract names **without `axone-` prefix**: `gov`, `logic`
- Other examples: `workflow`, `dependabot`, `README`, `make`, `deps`
- **If unsure, omit it**

## Examples

See [examples](./assets/examples.md) for comprehensive good and bad examples.

Quick reference:

```text
feat(gov): introduce quadratic voting mechanism
fix(handlers): prevent overflow in vote counting
refactor(state): clarify storage key naming
test(gov): validate error paths for unauthorized access
build(deps): enforce abstract-sdk 0.26.1
```

## Granularity

Rule: **One commit = one intention**

| Instead of... | Prefer... |
| - | - |
| One big mixed commit | Multiple focused commits |
| `feat: implement X and fix Y` | Two separate commits |
| Tests bundled with feature | Separate `test:` commit |
| Build changes with feature | Separate `build:` commit |

## What to Avoid

- ❌ Generic messages hiding what changed
- ❌ Explanations or rationale in the message
- ❌ Marketing language or inflated wording
- ❌ Multiple intentions in one commit
- ❌ Vague subjects like "improve", "update", "fix issue"

## Breaking Changes

Use `!` after type/scope:

```text
feat(msg)!: restructure ExecuteMsg variants
refactor(api)!: enforce stricter validation schema
```

## Commit Linting

Commits are validated using [commitlint](https://commitlint.js.org/). Ensuring lint passes is **mandatory**.

Validate locally:

```bash
npm i -g @commitlint/cli @commitlint/config-conventional
echo "feat(gov): introduce voting mechanism" | commitlint --extends @commitlint/config-conventional
```

## Quick Checklist

Before committing, verify:

- [ ] Uses a strong, precise verb
- [ ] Subject describes intent clearly
- [ ] Scope is present if it adds clarity
- [ ] One intention per commit
- [ ] No generic or vague wording
- [ ] Commit message passes linting
