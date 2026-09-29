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

- Added runtime release and Render deployment commit provenance to the authenticated live-state surface.

## Unreleased — v0.3.0 development

## [0.1.0] — Foundation

### Added
- Public GyLiber landing surface with About, Work and Links pages.
- Discreet member-access entry.
- GitHub OAuth Authorization Code authentication with PKCE.
- Explicit GitHub username allowlist for internal access.
- Private authenticated sessions with HttpOnly, Secure and SameSite controls.
- Protected Command Center route.
- Protected live-state API.
- Protected typed module registry API and dynamic dashboard rendering.
- Browser live-state polling with freshness display.
- Public health endpoint for deployment probes.
- CSP, X-Frame-Options, X-Content-Type-Options, Referrer-Policy, Cache-Control, Permissions-Policy, Cross-Origin-Opener-Policy and Cross-Origin-Resource-Policy headers.
- Production transport validation, HSTS policy and request-body limits.
- Request IDs and HTTP tracing.
- Controlled 404 page.
- Rust unit and HTTP-boundary tests.
- GitHub Actions CI with format, check, clippy, test and advisory verification.
- CI concurrency cancellation and execution timeout.
- Dependabot configuration for Cargo and GitHub Actions.
- Docker and Render deployment definitions.
- Requirements, system design, security baseline, data classification, backup/recovery, manual setup, governance and architecture decision records.

### Security boundary
v0.1.0 intentionally stores no banking records, production credentials, high-value trade secrets, personnel records, customer records or irreplaceable corporate archives.

### Known operational limitation
The v0.1 session store is in memory. Active sessions are lost on process restart. This is accepted only for the initial non-critical release and must be replaced by durable identity/session infrastructure before multi-instance production operation.

## Future
Subsequent releases will add modular resource registries, richer live operations, knowledge modules, persistent private data behind security gates, independent backups, stronger identity/passkeys, observability and production-grade deployment controls.
