# ADR-0007: Reproducible Rust Dependency Resolution

**Status:** Accepted  
**Date:** 2026-09-29  
**Release:** v0.2.0 development

## Context

The Command Center is security-sensitive and must be reproducible. A dependency manifest alone allows a future CI run to resolve different compatible dependency versions from the same source tree.

The repository had no committed `Cargo.lock`, while production-oriented CI already depended on deterministic advisory auditing.

## Decision

Commit the generated `Cargo.lock` and require locked dependency resolution in CI for the Rust checks that consume it:

- `cargo check --locked`
- `cargo clippy --all-targets --all-features --locked -- -D warnings`
- `cargo test --locked`
- `cargo audit` continues to use `--locked`

Dependency upgrades must therefore update `Cargo.toml` and `Cargo.lock` together.

## Consequences

### Positive
- CI cannot silently resolve a changed dependency graph.
- A reviewed commit records the exact Rust dependency graph used for verification.
- Advisory and build results become easier to reproduce.
- Dependency changes become explicit review units.

### Constraints
- Developers must regenerate the lockfile intentionally when dependencies change.
- The lockfile can create larger review diffs during dependency upgrades.
- A stale lockfile causes CI failure rather than silently updating dependencies.

## Policy

Dependency PRs should be verified with the same locked graph that will be used for release candidates. Large or security-sensitive upgrades should remain independently reviewable.
