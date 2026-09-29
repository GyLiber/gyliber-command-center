# GyLiber Command Center

> **GyLiber's digital headquarters:** a high-signal operational hub for systems, resources, knowledge and future company operations.

![Status](https://img.shields.io/badge/release-v0.1.0--foundation-slate)
![Backend](https://img.shields.io/badge/backend-Rust-orange)

## 1. Product

GyLiber Command Center is being built as a long-lived **command-center hub**, not as a conventional brochure website.

The public surface represents GyLiber as a company/brand. The authenticated surface is intended to become the operational control plane for GyLiber's digital resources, live system information, knowledge, documents, integrations and—only after explicit security gates—high-value company information.

The interface is deliberately calm, dark and information-forward. Its design target is:

> **more useful state and action per unit of human attention.**

## 2. Current release

### v0.1.0 Foundation

The first end-to-end vertical slice currently includes:

- public GyLiber landing page
- About / Work / Links public pages
- discreet member-access route
- GitHub OAuth Authorization Code flow with PKCE
- explicit GitHub member allowlist
- private authenticated session
- protected Command Center
- protected live-state API
- browser polling of live state every 15 seconds
- health endpoint for deployment probes
- security response headers
- request IDs and structured request tracing
- custom 404 page
- Rust tests for route/authentication boundaries
- GitHub Actions CI
- dependency-update automation
- Docker deployment
- Render deployment blueprint
- requirements, architecture, security, governance and operational runbooks

### Deliberately absent from v0.1.0

The first release does **not** authorize storage of:

- banking credentials
- unrestricted banking/financial records
- production cloud secrets
- high-value trade secrets
- personnel records
- customer records
- irreplaceable corporate archives

That boundary is intentional.

## 3. Architecture

```text
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

## 4. Technology baseline

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
| Deployment | Docker / Render for initial environment |
| Observability direction | tracing + OpenTelemetry-compatible architecture |
| Browser surface | standards-based HTML/CSS/JS in v0.1 |

Dependency versions are maintained in `Cargo.toml` and `Cargo.lock` when dependency resolution is generated.

## 5. Security model

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
- automated security-oriented tests
- data-classification gates
- independent backup/recovery requirements for critical data

**Important:** no software language or framework guarantees an unbreachable system. Rust addresses important classes of implementation risk; complete security depends on the whole system and its operation.

## 6. Data classification

Future information is classified as:

`PUBLIC` → `INTERNAL` → `CONFIDENTIAL` → `RESTRICTED` → `CRITICAL`

Critical information requires stronger identity, authorization, audit, storage, backup, recovery and operational controls.

See:

- `docs/security/DATA_CLASSIFICATION.md`
- `docs/security/SECURITY.md`
- `docs/operations/BACKUP_AND_RECOVERY.md`

## 7. Engineering workflow

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

## 8. CI/CD

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

## 9. Manual setup

Gyile/GyLiber should not need to infer infrastructure setup from source code.

Start with:

`docs/operations/MANUAL_SETUP.md`

That runbook explains Rust/Cargo, GitHub OAuth App creation, callback URLs, environment configuration, secrets, and which steps can safely be deferred.

## 10. Hosting and recovery

The first live environment is intentionally low-cost.

Hosting is not the archive.

The mature platform will use independent recovery mechanisms so loss of the laptop, application host, database or one storage provider does not eliminate GyLiber's irreplaceable information.

See `docs/operations/BACKUP_AND_RECOVERY.md`.

## 11. Extensibility

A future feature may become:

1. an internal module,
2. a bounded backend/domain service,
3. an independently deployed application, or
4. a linked companion site.

The Command Center remains the navigation/control plane when that split occurs.

This permits future modules such as creative knowledge/trivia experiences, project operations, asset observability, intellectual-property records, staff operations and financial representations without forcing every function into one page or one process.

## 12. AI-assisted engineering

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

## 13. Intellectual property

This repository is currently public during the early development phase.

Public visibility does not grant a general license to reuse GyLiber intellectual property.

See `LICENSE` for the current repository-level rights notice. Third-party dependencies remain governed by their own licenses.

## 14. Living documentation

The following documents are intentionally living documents:

- `docs/requirements/REQUIREMENTS.md`
- `docs/architecture/DESIGN.md`
- `docs/security/SECURITY.md`
- `docs/security/DATA_CLASSIFICATION.md`
- `docs/operations/MANUAL_SETUP.md`
- `docs/operations/DEPLOYMENT.md`
- `docs/operations/BACKUP_AND_RECOVERY.md`
- `docs/governance/DEVELOPMENT.md`
- `docs/governance/KNOWLEDGE_GAPS.md`

The architecture is expected to evolve as implementation evidence accumulates.