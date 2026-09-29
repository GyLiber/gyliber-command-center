# Changelog

## [0.1.0] — Foundation

### Added
- Public GyLiber landing surface with About, Work and Links pages.
- Discreet member-access entry.
- GitHub OAuth Authorization Code authentication with PKCE.
- Explicit GitHub username allowlist for internal access.
- Private authenticated sessions with HttpOnly, Secure and SameSite controls.
- Protected Command Center route.
- Protected live-state API.
- Browser live-state polling with freshness display.
- Public health endpoint for deployment probes.
- CSP, X-Frame-Options, X-Content-Type-Options, Referrer-Policy, Cache-Control, Permissions-Policy and Cross-Origin-Opener-Policy headers.
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
