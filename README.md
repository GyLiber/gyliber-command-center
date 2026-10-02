# GyLiber Command Center

> **GyLiber's digital headquarters:** a high-signal operational hub for systems, resources, knowledge and future company operations.
>
> **Ever Toward Liberation.**

![Status](https://img.shields.io/badge/release-v0.2.0--live-product-foundation-slate)
![Backend](https://img.shields.io/badge/backend-Rust-orange)

## 1. Product

GyLiber Command Center is being built as a long-lived **command-center hub**, not as a conventional brochure website.

The public surface represents GyLiber as a company/brand. The authenticated surface is intended to become the operational control plane for GyLiber's digital resources, live system information, knowledge, documents, integrations, active/planned engagements and—only after explicit security gates—high-value company information.

The interface is deliberately calm, dark and information-forward. Its design target is:

> **more useful state and action per unit of human attention.**

## 2. Current release

### v0.2.0 Live product foundation

The current live-product foundation includes:

- public GyLiber landing page
- About / Work / Links public pages
- discreet member-access route
- GitHub OAuth Authorization Code flow with PKCE
- explicit GitHub member allowlist
- private authenticated session
- protected Command Center
- protected live-state API
- protected typed module registry API
- dynamic module rendering from the internal registry
- reserved Contracts & Engagements capability for future secure tracking of active and planned client engagements
- browser polling of live state every 15 seconds
- health endpoint for deployment probes
- security response headers
- request IDs and structured request tracing
- custom 404 page
- Rust tests for route/authentication boundaries
- GitHub Actions CI
- dependency-update automation
- Docker deployment
- initial managed-host deployment
- runtime release/build provenance in the protected Live State Monitor
- safe public-only deployment mode while member authentication is unconfigured
- requirements, architecture, security, governance and operational runbooks

### Mathematical playground development

The member Command Center now registers `/command/math-playground`: four playable, repository-backed demonstrations, source upload, configured AI concept drafting, formal reveal, private exhibit persistence and source-free Git publishing. AI authoring and publishing require explicit hosting activation; unavailable settings are reflected in the page. General-purpose arbitrary AI code execution is not enabled.

See [playground activation and acceptance](docs/operations/MATH_PLAYGROUND.md) and [ADR-0010](docs/architecture/ADR-0010-bounded-mathematical-playground.md). Exhibit engine 0.1.0, formal reveal 0.2.0 and deferred proof 0.3.0 are independent capability versions.

### Deliberately absent from v0.2.0

The first release does **not** authorize storage of:

- banking credentials
- unrestricted banking/financial records
- production cloud secrets
- high-value trade secrets
- personnel records
- customer records
- irreplaceable corporate archives

That boundary is intentional.

## 3. Canonical public address

The application is designed to expose a **GyLiber-owned canonical domain** as its public identity.

The infrastructure provider's generated hostname is an implementation detail and is not the intended business-card, client-facing or brand address.

### Target presentation

The preferred final presentation is:

`https://gyliber.com/`

subject to GyLiber ownership and availability of that domain.

A subdomain such as `https://hq.gyliber.com/` is the documented fallback when the root domain is reserved for a separate public corporate site.

The canonical domain must remain independent of the underlying hosting provider so infrastructure can be changed without changing GyLiber's public identity.

## 4. Architecture

```
                         INTERNET
                             |
              +--------------+--------------+
              |                             |
         PUBLIC WEB                 MEMBER AUTHENTICATION
              |                             |
     +--------+--------+                    v
     | About / Work    |             +-------------+
     | Links / Brand   |             | Command Hub |
     +-----------------+             +------+------+
                                             |
                                  +----------+----------+
                                  |                     |
                             live state             future modules
                                  |                     |
                              API layer       +---------+---------+
                                              | knowledge / IP    |
                                              | resources         |
                                              | operations        |
                                              | finance*          |
                                              | people*           |
                                              +---------+---------+
                                                        |
                                                   data services
                                                        |
                                       +----------------+----------------+
                                       |                                 |
                                  PostgreSQL*                      Object storage*
```

`*` denotes a future security-gated capability.

The backend is the trust authority. Browser state is never treated as authorization.

## 5. Technology baseline

The current implementation intentionally excludes Python from the application stack.

| Concern | Baseline |
|---|---|
| Application language | Rust |
| HTTP framework | Axum |
| Async runtime | Tokio |
| HTTP client | reqwest + rustls |
| Authentication protocol | GitHub OAuth + PKCE |
| Session layer | tower-sessions |
| Persistence baseline | PostgreSQL |
| Object storage baseline | S3-compatible |
| CI/CD | GitHub Actions |
| Deployment | Docker on an abstracted managed hosting environment |
| Observability direction | tracing + OpenTelemetry-compatible architecture |
| Browser surface | standards-based HTML/CSS/JS |

Dependency versions are maintained in `Cargo.toml` and the committed `Cargo.lock`; CI uses locked resolution for reproducible verification.

## 6. Security model

Security starts from the first commit.

Core controls include:

- deny-by-default protected routes
- explicit member allowlisting
- OAuth state + PKCE
- private session protection
- HttpOnly / Secure / SameSite cookie controls
- no secret values in source control
- CSP and browser security headers
- disabled outbound HTTP redirects for the OAuth client
- dependency/advisory checks
- CodeQL static analysis and Gitleaks secret scanning
- automated security-oriented tests
- data-classification gates
- independent backup/recovery requirements for critical data

**Important:** no software language or framework guarantees an unbreachable system. Rust addresses important classes of implementation risk; complete security depends on the whole system and its operation.

## 7. Data classification

Future information is classified as:

`PUBLIC` → `INTERNAL` → `CONFIDENTIAL` → `RESTRICTED` → `CRITICAL`

Critical information requires stronger identity, authorization, audit, storage, backup, recovery and operational controls.

See:

- `docs/security/DATA_CLASSIFICATION.md`
- `docs/security/SECURITY.md`
- `docs/operations/BACKUP_AND_RECOVERY.md`

## 8. Engineering workflow

Every commit should be a coherent unit of work.

Examples:

```text
chore: initialize repository structure
feat: add public command center shell
test: cover protected state boundary
fix: reject unauthorized resource access
refactor: isolate resource status module
docs: record architecture decision
```

Correct earlier work may be superseded when implementation evidence proves an earlier assumption incorrect. The correction should remain visible in history.

## 9. CI/CD

CI is part of the product.

Current verification includes:

```text
cargo fmt
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo audit
```

The workflow is manually dispatchable and runs automatically for repository changes.

The project is intended to evolve toward stronger release verification such as:

- SBOM generation
- artifact provenance
- container vulnerability scanning
- CodeQL
- policy enforcement
- deployment health verification
- restore/recovery tests

## 10. Manual setup

Gyile/GyLiber should not need to infer infrastructure setup from source code.

Start with:

`docs/operations/MANUAL_SETUP.md`

That runbook explains Rust/Cargo, GitHub OAuth App creation, callback URLs, environment configuration, secrets, canonical-domain binding and which steps can safely be deferred.

## 11. Hosting and recovery

The initial hosted service is intentionally low-cost and replaceable.

**The hosting provider is not the product identity and is not the archive.**

The canonical public domain belongs to GyLiber. DNS points that domain to the current hosting service, allowing the underlying provider to change without requiring a new business-card address.

The mature platform will use independent recovery mechanisms so loss of the laptop, application host, database or one storage provider does not eliminate GyLiber's irreplaceable information.

See `docs/operations/BACKUP_AND_RECOVERY.md`.

## 12. Extensibility

A future feature may become:

1. an internal module,
2. a bounded backend/domain service,
3. an independently deployed application, or
4. a linked companion site.

The Command Center remains the navigation/control plane when that split occurs.

This permits future modules such as creative knowledge/trivia experiences, project operations, asset observability, intellectual-property records, staff operations and financial representations without forcing every function into one page or one process.

## 13. AI-assisted engineering

AI is intentionally used as an engineering accelerator for research, implementation, testing, documentation and refactoring.

AI output is never treated as authoritative.

Human responsibility remains with GyLiber and the project maintainers for:

- requirements
- architecture
- security
- dependency selection
- intellectual property
- verification
- releases
- production operations

## 14. Intellectual property

This repository is currently public during the early development phase.

Public visibility does not grant a general license to reuse GyLiber intellectual property.

See `LICENSE` for the current repository-level rights notice. Third-party dependencies remain governed by their own licenses.

## 15. Living documentation

The following documents are intentionally living documents:

- `docs/requirements/REQUIREMENTS.md`
- `docs/architecture/DESIGN.md`
- `docs/architecture/MATH_PLAYGROUND_DESIGN.md` — source-driven creative mathematics design, with bounded implementation in ADR-0010
- `docs/architecture/ADR-0009-canonical-public-domain.md`
- `docs/architecture/ADR-0010-bounded-mathematical-playground.md`
- `docs/operations/MATH_PLAYGROUND.md`
- `docs/security/SECURITY.md`
- `docs/security/DATA_CLASSIFICATION.md`
- `docs/operations/MANUAL_SETUP.md`
- `docs/operations/DEPLOYMENT.md`
- `docs/operations/BACKUP_AND_RECOVERY.md`
- `docs/operations/CLIENT_DEMONSTRATION_BASELINE.md`
- `docs/operations/NEXT_DEVELOPMENT_STEPS.md`
- `docs/governance/DEVELOPMENT.md`
- `docs/governance/DEVELOPMENT_HANDOFF.md`
- `docs/governance/KNOWLEDGE_GAPS.md`

The architecture is expected to evolve as implementation evidence accumulates.
