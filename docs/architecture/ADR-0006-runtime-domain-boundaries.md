# ADR-0006: Runtime Domain Boundaries

**Status:** Accepted  
**Date:** 2026-09-29  
**Release:** v0.2.0 development

## Context

The Command Center has outgrown a single source file as the application continues to gain authenticated modules, external integrations and security controls.

The application is expected to support a professional team of approximately 6–10 developers.

## Decision

Separate the runtime into explicit Rust modules with single primary responsibilities:

- `config`: environment configuration, runtime security validation, HTTP client construction and session-key loading.
- `auth`: GitHub OAuth, PKCE, member identity and member-allowlist behavior.
- `modules`: Command Center module catalog.
- `state`: low-risk live system snapshot.
- `repository`: GitHub repository observability adapter.
- `resources`: digital resource registry.
- `main`: process startup, route composition, public handlers and remaining application composition.

This is a modular-monolith boundary. It does not create network services merely to obtain file separation.

## Rules

A module should own:
- its domain types;
- its domain-specific validation;
- its external provider interaction where applicable;
- its focused unit tests.

Cross-domain policy remains explicit. The browser remains untrusted and authorization remains server-side.

## Consequences

### Positive
- Smaller review surfaces.
- Clearer ownership for future developers.
- Lower coupling between integrations and the HTTP composition root.
- Easier future extraction into services when evidence justifies it.

### Constraints
- Module boundaries are not security boundaries by themselves.
- The application remains one process in v0.2.0.
- Cross-module state must use explicit typed interfaces.

## Future

Authentication, data persistence and high-impact domains may eventually acquire separate crates/services where security isolation, scaling, ownership or operational fault isolation justifies the boundary.
