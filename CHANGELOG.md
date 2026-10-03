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

- Recorded Gyile's failed synthetic metric-space generation and prioritized external authoring, reviewed repository packages and provider-independent playback in ADR-0011. Added a reusable external authoring template and updated Next Steps/runbook/handoff; no runtime or automatic code-import change is made by this plan.

- Sol resumed on 2026-10-03 and added dark executable exhibit bundle 0.2.0: subdued Canvas palettes, matching formal/source/control surfaces, explicit animation opt-in and no animation scheduling while paused or hidden.
- Retained original 0.1.0 renderer/demo bytes and private saved packages, added authenticated versioned demo renderers and an explicit dim/original choice for legacy playback. Downloads retain original code and use the actual bundle version in their filename.
- Extended old/new package digest, PostgreSQL preservation and browser coverage; CI records synthetic desktop/mobile screenshots. Mathematical algorithms and formal reveal are unchanged; theorem proofs remain deferred to 0.3.0.
- Updated Next Steps and Sol’s handoff. Gyile reported live AI retest not yet performed; successful authoring/publication and dark hosted comfort remain acceptance gates.

- Corrected new-project Gemini setup to gemini-3.5-flash-lite after the live model-unavailable error and Google's published restriction of Gemini 2.5 to prior active users. No automatic model fallback is introduced.
- Removed candidateCount from Gemini requests for Gemini 3 compatibility, retaining exactly-one-candidate response validation and the existing bounded structured-output contract. Updated private Render recovery steps; live generation still needs observation after redeploy.

- Use Gemini's established responseMimeType/responseJsonSchema output configuration while retaining store: false. The live API recognizes the earlier format too, so this compatibility change is not a proven production root-cause fix. OpenAI request behavior is unchanged.
- Distinguish Gemini's HTTP-400 invalid-key reason, request rejection, unavailable model and timeout from transient provider failure. Error inspection is bounded and exposes/logs only fixed codes and status metadata.
- Updated deployed-playground evidence and provider recovery instructions. Successful live generation/publication still require observation with the deployed credentials.
- Documented Gyile's next-minor preference for calmer dark visuals matching the site, covering Canvas and formal/source panels. It was not included in the provider fix and is implemented by the later visual work above.

- Added explicit Gemini authoring as a free-tier pilot alternative to OpenAI, with provider-bound consent, unpaid-service data-use disclosure, quota/access messages and no automatic paid fallback.
- Corrected the browser fixture's decimal range input and prevented its exception details from being returned as HTML.

- Sol added the member Mathematical Playground: playful Canvas scenes, bounded `.tex` intake, AI concept drafting, a general formal reveal, private PostgreSQL derivatives and exact-byte, source-free Git artifact publishing.
- Added reviewed Pi, reciprocal-sequence, permutation and mnemonic demonstrations that do not require AI credentials.
- Added request-token checks, owner isolation, attempted-request budgets, draft expiry, source anchors, artifact digests and explicit unavailable/review states.
- The initial exhibit bundle 0.1.0 and reveal capability 0.2.0 remain separate from application releases; theorem proofs are deferred to exhibit capability 0.3.0.
- Added mathematical invariant, browser-fixture and PostgreSQL/HTTP-boundary checks. Hosted authoring/publishing requires the activation gates in `docs/operations/MATH_PLAYGROUND.md`.

- Added the reserved Contracts & Engagements module to the authenticated registry as a Confidential, security-gated capability for future active/planned client engagement tracking.
- Added the GyLiber slogan, **Ever Toward Liberation**, to the public Command Center identity.
- Documented contract/engagement record requirements, classification and security prerequisites without onboarding real contract data.

- Added a PostgreSQL-backed production session-store implementation with migration support and fail-closed production configuration.
- Added an ephemeral PostgreSQL 18 CI service and an automated session create/save/load/delete round-trip test.
- Documented the current temporary free PostgreSQL environment and its 2026-10-30 expiry.
- Configured the private/internal `DATABASE_URL` in the Render web service environment without recording the secret value in source control.
- Recorded the first controlled deployment failure caused by the Docker build context omitting `migrations/0001_sessions.sql`.
- PR #22 supplied the Docker build-context correction and was merged after successful CI/Security. Subsequent hosted playground deployment was observed; full provider/publishing acceptance remains pending as recorded in the playground runbook.
- Sol resumed development from Luna's PR #22 checkpoint on 2026-10-01; see `docs/governance/DEVELOPMENT_HANDOFF.md`.
- Production builds now use the committed `Cargo.lock`, locked resolution and the same Rust 1.98.1 toolchain as CI.
- Added a production-container CI gate covering fail-closed configuration, PostgreSQL migration, HTTP health, static assets, anonymous access boundaries and restart with persisted session metadata.
- Deferred the canonical domain purchase without changing the long-term GyLiber-owned domain strategy.

The v0.3 development line is being implemented incrementally. See:

`docs/operations/NEXT_DEVELOPMENT_STEPS.md`

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
- Docker and managed-host deployment definitions.
- Requirements, system design, security baseline, data classification, backup/recovery, manual setup, governance and architecture decision records.

### Security boundary
v0.1.0 intentionally stores no banking records, production credentials, high-value trade secrets, personnel records, customer records or irreplaceable corporate archives.

### Known operational limitation
The v0.1 session store is in memory. Active sessions are lost on process restart. This is accepted only for the initial non-critical release and must be replaced by durable identity/session infrastructure before multi-instance production operation.

## Future
Subsequent releases will add modular resource registries, richer live operations, knowledge modules, persistent private data behind security gates, independent backups, stronger identity/passkeys, observability and production-grade deployment controls.
