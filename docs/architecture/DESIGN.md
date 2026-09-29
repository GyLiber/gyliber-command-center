# Software Design Document — GyLiber Command Center v1.0.0

## 1. Architecture decision

The product is a modular command-center platform, not a generic corporate website and not an early microservice maze.

The first implementation uses a **Rust/Axum backend serving a modern browser client**. The backend remains the trust authority; the browser is treated as untrusted.

The architecture may later introduce a separate frontend framework, WASM modules, or independently deployed services when evidence justifies the boundary.

## 2. Current 2026 technology baseline

| Area | Baseline |
|---|---|
| Backend language | Rust 1.98.1 |
| HTTP | Axum 0.8.x + Tower ecosystem |
| Database | PostgreSQL 18.x when persistence is introduced |
| Frontend | Web platform + TypeScript/JavaScript as needed; browser code is never trusted for authorization |
| Authentication | GitHub OAuth + PKCE for current member boundary; OIDC/WebAuthn/passkeys in mature design |
| Real-time | SSE first; WebSockets only for genuine bidirectional needs |
| Object storage | S3-compatible storage; migration-friendly |
| CI/CD | GitHub Actions |
| Deployment auth | OIDC federation where provider supports it |
| Observability | OpenTelemetry-compatible |
| Testing | Rust unit/integration + browser/API E2E |
| Security | dependency audit + secret scanning + SBOM/security checks |

Python is deliberately excluded.

Rust 1.98.1 is the current stable release used by this design baseline. Axum 0.8.9 and PostgreSQL 18.6 are current checked baselines as of 2026-09-29. Next.js 16.3 is current, but the v0.1 implementation intentionally avoids creating an additional server runtime until its value is demonstrated.

## 3. Public identity and infrastructure boundary

The canonical public origin belongs to GyLiber, not to the hosting provider.

```
                      GYLIBER-OWNED PUBLIC DOMAIN
                               |
                               v
                        managed hosting edge
                               |
                         Rust application
```

The public domain is the stable product address. The hosting service's generated hostname, deployment IDs and dashboard URLs are operational references only.

Changing hosting providers should not require changing printed business cards, client materials or the public authentication contract except for DNS/certificate and callback reconfiguration.

The preferred public address is `https://gyliber.com/` when available and controlled by GyLiber. A short subdomain such as `https://hq.gyliber.com/` is the fallback when the root domain is used for another corporate surface.

## 4. Trust boundaries

```text
Internet
   |
   +-- Canonical Public Web
   |
   +-- Authentication / Identity Boundary
             |
             +-- Internal Command Center
                    |
                    +-- Domain modules
                    |
                    +-- Data services
                          +-- PostgreSQL
                          +-- object storage
                          +-- audit/event store
                          +-- independent backup
```

The backend authorizes every protected operation independently of UI state.

## 5. Domain modules

The first runtime domain boundary is the module registry. It defines typed module identity, route, availability, data classification and live-state capability without persisting business data. The protected `/api/modules` endpoint is the contract consumed by the Command Center dashboard. New modules should be added through this registry before acquiring larger UI/backend boundaries.

Planned bounded domains:
- identity and authorization
- resource registry
- operations/live state
- knowledge/trivia
- documents/IP
- contracts/engagements (confidential)
- finance (restricted)
- people/staff (restricted)
- integrations/secrets (restricted)
- audit/events
- system administration

The Contracts & Engagements domain is the planned record of active and prospective GyLiber client engagements. It should separate contract metadata from controlled document content and enforce classification-aware, role-limited access with durable audit events.

A domain may remain in the primary application or be extracted when security, scaling, deployment ownership or fault isolation makes extraction worthwhile.

## 6. Information freshness

Every live-state representation should eventually expose:
- source
- last observed time
- freshness status
- transformation/derivation
- authorization context

The UI must distinguish live, recently observed, stale and unknown.

## 7. Data classification

`PUBLIC`, `INTERNAL`, `CONFIDENTIAL`, `RESTRICTED`, `CRITICAL`.

Classification controls storage, display, authorization, audit, retention and recovery. Contract and engagement records are Confidential by default; client-identifying, financial, credential or other higher-risk fields may require Restricted or Critical handling.

## 8. Versioning

Semantic versioning is used:
- v0.1.0: first working vertical slice
- v0.2.0: live product foundation
- v0.x: additive/refining work
- v1.0.0: first mature public/private platform contract
- later versions remain extensible without assuming a final architecture

Architecture changes are permitted when new evidence invalidates earlier assumptions. Material changes require an Architecture Decision Record.

## 9. Throughput-oriented UX

The interface should behave like an operational room:
- dense but legible state
- persistent navigation
- priority state first
- status/freshness visible
- direct drill-down
- minimal decorative friction
- calm dark palette with centralized design tokens
- branding assets isolated from application logic

The aim is not maximum information density; it is maximum **useful information/action per attention unit**.

## 10. Transport and input hardening

The HTTP boundary disables outbound redirects in the OAuth client, applies secure browser headers, applies request-body limits, and validates production transport configuration before startup. Production requires HTTPS callback URLs and secure cookies.

## 11. Disaster recovery

Primary hosting is not the archive.

Critical production data eventually requires an independently recoverable copy, separate credentials, retention/versioning, restore testing and defined RPO/RTO.

Free/low-cost hosting is an early engineering environment, not automatic authorization to store critical company records.

## 12. Release gates

No release may be represented as “secure” merely because tests pass. High-impact private data requires evidence across application security, identity, infrastructure, data protection, monitoring, backups, recovery and independent review.
