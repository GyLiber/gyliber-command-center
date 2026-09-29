# GyLiber Command Center — Next Development Plan

**Document status:** Planned  
**Planning date:** 2026-09-29  
**Current release:** v0.2.0 Live Product Foundation  
**Next development line:** v0.3.x operational hardening  
**Product principle:** maximize useful state and action per unit of human attention.

## 1. Current position

The Command Center now has a verified live vertical slice:

- public GyLiber landing surface
- About / Work / Links
- member-access entry
- GitHub OAuth + PKCE boundary
- explicit member allowlist
- protected Command Center
- typed module registry
- protected live-state surface
- repository/resource observability
- public health endpoint
- security headers and request tracing
- CI + security scanning
- Docker-based deployment
- runtime release/build provenance
- safe public-only bootstrap
- live managed-host deployment
- deployment and demonstration runbooks
- documented provider-independent public-domain strategy

The current environment remains an engineering/demo environment. It is **not** an authorization to load critical company information.

## 2. Immediate external dependency

### Canonical public address

The next non-code infrastructure task is to establish the GyLiber-owned canonical public address.

Preferred:

`https://gyliber.com/`

Fallback:

`https://hq.gyliber.com/`

The final choice depends on domain ownership and intended corporate information architecture.

Once established, the domain is attached to the current service, DNS is configured, TLS is verified, and the GitHub OAuth callback plus `GITHUB_REDIRECT_URL` are migrated to the canonical origin.

The provider-generated hostname then becomes an internal operational reference only.

## 3. v0.3 — Operational hardening

The next development line should strengthen the platform before expanding the amount of information it handles.

### 3.1 Durable application/session boundary

Replace the current in-memory session limitation with a durable session architecture backed by managed persistence.

Acceptance evidence:

- sessions survive a service restart;
- session expiry/revocation is explicit;
- secrets remain outside source control;
- failure of the persistence layer fails safely;
- tests cover session lifecycle and failure behavior.

### 3.2 Database foundation

Introduce PostgreSQL only as a controlled platform capability, not as a reason to load sensitive company data immediately.

Scope:

- versioned migrations;
- connection configuration;
- least-privilege database account;
- startup/readiness behavior;
- migration verification;
- database health diagnostics.

Acceptance evidence:

- deterministic migration from an empty database;
- application startup with persistence available;
- safe behavior when persistence is unavailable;
- CI coverage for migration assumptions.

### 3.3 Audit/event persistence

Move beyond process-local audit signals toward a durable, queryable event boundary.

Initial event classes:

- authentication lifecycle;
- protected-access denial;
- external-source failure;
- administrative/configuration changes.

The first persisted audit store should contain operational metadata, not sensitive business payloads.

Acceptance evidence:

- events have stable codes;
- timestamps are authoritative;
- actor identity is represented appropriately;
- sensitive values are not logged;
- retention expectations are documented.

### 3.4 Contract and engagement record boundary

Introduce a security-gated internal capability for recording active and planned GyLiber client engagements. The first design should cover contract/engagement identity, lifecycle status, key dates, scope summary, commercial milestones and links to controlled agreement documents without storing real records in the current release.

Default classification is **CONFIDENTIAL**. Any field containing more sensitive client, financial, credential or personnel information must be classified separately and may require **RESTRICTED** or **CRITICAL** handling.

Acceptance evidence:

- records are available only through authenticated, authorized member operations;
- authorization is enforced server-side at the resource/action boundary;
- sensitive payloads are excluded from public pages and unauthenticated APIs;
- create/update/delete operations produce durable audit events;
- agreement files are stored in controlled object storage rather than ordinary source-controlled assets;
- retention, deletion and backup/recovery expectations are documented before real contracts are onboarded.

### 3.5 Deployment verification

Make deployment health a first-class release gate.

Target loop:

`commit → CI → security → merge → deploy → health verification → release evidence`

Acceptance evidence:

- deployment result is captured;
- `/api/health` is checked automatically or through an explicit runbook;
- the running release identifies its source revision;
- failures are visible without inspecting source code manually.

## 4. v0.3 — Integration boundary

After the reliability foundation, expand the Command Center's usefulness through bounded integrations.

### 4.1 Typed integration adapters

Create a common backend contract for read-only external sources.

Each adapter should expose:

- source identity;
- fetched timestamp;
- freshness state;
- normalized status;
- bounded timeout;
- bounded result size;
- explicit unavailable/error state;
- authorization context.

No integration should silently become a generic proxy to the internet.

### 4.2 Repository and resource observability

Evolve the current repository/resource monitors into reusable typed capabilities.

The goal is not simply “more dashboards.” The goal is a common operational model that lets new sources plug into the Command Center without rewriting the entire UI.

### 4.3 Module lifecycle

Formalize module metadata:

- identity;
- display priority;
- route;
- classification;
- availability;
- freshness requirements;
- required permissions.

The registry remains the control-plane contract.

## 5. v0.4 — Command-center experience

Only after the backend contracts are stable should the UI become materially richer.

Potential capabilities:

- operational overview;
- event timeline;
- source freshness board;
- actionable alerts;
- resource drill-down;
- configurable module ordering;
- compact “what changed” views.

The interface should remain calm and high-signal rather than turning into a collection of decorative widgets.

## 6. v0.5+ — Identity and security maturation

Before any high-impact private information is onboarded:

- evaluate an identity architecture beyond the current GitHub-only boundary;
- introduce stronger authentication such as OIDC and/or WebAuthn/passkeys where justified;
- establish role-based authorization;
- centralize policy evaluation;
- define sensitive-operation approval/audit controls;
- implement stronger secret management;
- add SBOM and container security verification;
- perform independent security review.

No critical data should enter the system before these controls have evidence behind them.

## 7. v1.0 — Mature platform contract

v1.0 should represent a stable platform contract rather than simply “more features.”

Expected platform capabilities:

- stable canonical public identity;
- mature member identity and authorization;
- durable operational state;
- typed resource and integration registry;
- audit/event timeline;
- reliable observability;
- durable object storage;
- independent backup and recovery;
- documented RPO/RTO by data class;
- restore testing;
- modular knowledge/resource/operations surfaces;
- extraction of independently deployed services only where an actual boundary is justified.

## 8. Architectural guardrails

The following remain in force throughout development:

1. **Backend is the trust authority.**
2. **Protected access is deny-by-default.**
3. **The browser never becomes the authorization source.**
4. **Critical data remains gated until infrastructure, identity, audit and recovery evidence exist.**
5. **The hosting provider is replaceable infrastructure, not product identity.**
6. **No microservice is introduced merely for architectural appearance.**
7. **Every material feature needs a documented boundary and automated acceptance evidence.**
8. **When evidence invalidates an earlier design assumption, correct it and preserve the reasoning in history.**

## 9. Development order

```text
CANONICAL DOMAIN
      ↓
DURABLE SESSIONS
      ↓
POSTGRES FOUNDATION
      ↓
AUDIT/EVENT PERSISTENCE
      ↓
CONTRACT / ENGAGEMENT RECORD BOUNDARY
      ↓
DEPLOYMENT VERIFICATION
      ↓
TYPED INTEGRATION BOUNDARY
      ↓
RESOURCE / MODULE OBSERVABILITY
      ↓
RICHER COMMAND-CENTER UX
      ↓
STRONGER IDENTITY + AUTHORIZATION
      ↓
BACKUP / RECOVERY EVIDENCE
      ↓
v1.0 PLATFORM CONTRACT
```

The order is intentional: **reliability and control precede sensitive information and feature scale.**

## 10. Definition of progress

A milestone is considered complete when it produces all of the following:

- implementation artifact;
- documentation update;
- automated verification;
- source commit;
- deployment evidence when applicable;
- visible product behavior when applicable;
- explicit known limitations.

A feature that exists only in source code but cannot be demonstrated or verified is not treated as finished.
