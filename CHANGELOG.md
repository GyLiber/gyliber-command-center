# Changelog

## [0.2.0] — Live product foundation

- Centralized authenticated-member access policy for protected browser and API surfaces.
- Added explicit tests for page redirect and API authentication failure semantics.
- Added ADR-0008 documenting the authentication-policy boundary.

- Committed Cargo.lock for reproducible Rust dependency resolution.
- Updated CI to enforce locked dependency resolution for check, Clippy and tests.
- Added the reproducible dependency architecture decision record.

- Added a typed Digital Assets resource registry.
- Added a protected resource registry API and browser surface.
- Registered the resource registry as a first-class Command Center module.

- Added the first typed Command Center module registry.
- Added the protected Live State Monitor with server observation timestamps.
- Added live Repository Monitor integration for the public GyLiber Command Center GitHub repository.
- Added bounded upstream timeouts and explicit unavailable-state handling.
- Hardened production transport configuration, HSTS behavior and request-body limits.
- Added architecture and deployment documentation for the new modules and hosting constraints.

- Added runtime release and managed-host deployment commit provenance to the authenticated live-state surface.
- Added safe public-only deployment bootstrap while keeping protected routes denied until GitHub OAuth is configured.
- Added a public live-health indicator, dashboard build identity and the client demonstration baseline.
- Corrected the Docker runtime image to include system CA certificates required by rustls outbound TLS.
- Documented the canonical-domain strategy so the public GyLiber identity remains independent of the hosting provider.

## Unreleased — v0.3.0 development

- Added the reserved Contracts & Engagements module to the authenticated registry as a Confidential, security-gated capability for future active/planned client engagement tracking.
- Added the GyLiber slogan, **Ever Toward Liberation**, to the public Command Center identity.
- Documented contract/engagement record requirements, classification and security prerequisites without onboarding real contract data.

- Added a PostgreSQL-backed production session-store implementation with migration support and fail-closed production configuration.
- Added an ephemeral PostgreSQL 18 CI service and an automated session create/save/load/delete round-trip test.
- Documented the current temporary free PostgreSQL environment and its 2026-10-30 expiry.
- Configured the private/internal `DATABASE_URL` in the Render web service environment without recording the secret value in source control.
- Recorded the first controlled deployment failure caused by the Docker build context omitting `migrations/0001_sessions.sql`.
- PR #22 adds the required Docker build-context correction; v0.3 deployment activation remains pending successful verification and redeployment.
- Deferred the canonical domain purchase without changing the long-term GyLiber-owned domain strategy.

The v0.3 development line is being implemented incrementally. See:

`docs/operations/NEXT_DEVELOPMENT_STEPS.md`

Development is temporarily paused at the PR #22 verification/deployment-correction checkpoint.
