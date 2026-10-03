# GyLiber Command Center — Next Development Plan

**Document status:** Active roadmap  
**Planning date:** 2026-09-30  
**Last development update:** 2026-10-03
**Current release:** v0.2.0 Live Product Foundation  
**Next development line:** v0.3.x operational hardening  
**Product principle:** maximize useful state and action per unit of human attention.

## 1. Current position

The Command Center has a verified live vertical slice:

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

The current live service remains on the v0.2 product foundation with the durable-session/container correction merged and subsequent hosted activation observed. The current environment remains an engineering/demo environment. It is **not** an authorization to load critical company information.

## 2. Deferred external dependency

### Canonical public address

The canonical GyLiber-owned public domain remains the long-term target:

- preferred: `https://gyliber.com/`
- fallback: `https://hq.gyliber.com/`

**Domain purchase is deferred for now because of budget.** Development does not block on the purchase. The current managed-host URL remains the temporary public address until a GyLiber-owned domain is secured.

When the domain is eventually secured, perform the documented DNS, TLS and GitHub OAuth callback migration, then remove the provider-generated hostname from client-facing material.

## 3. v0.3 — Operational hardening

The next development line should strengthen the platform before expanding the amount of information it handles.

### 3.1 Durable application/session boundary

**Implementation status: merged through PR #22; subsequent hosted deployment observed. Full signed-in session continuity across a hosted restart remains an acceptance check.**

The application now uses a PostgreSQL-backed session store in production and retains an in-memory store only for local/development operation. Production refuses to start without `DATABASE_URL`.

Automated CI exercises the PostgreSQL store against an ephemeral PostgreSQL 18 service and verifies session create/save/load/delete behavior.

The first controlled Render deployment of the merged implementation failed at Docker build time because the `migrations/` directory was omitted from the image build context. Merged PR #22 adds the missing `COPY migrations ./migrations` step.

Acceptance evidence:

- sessions survive a service restart;
- session expiry/revocation is explicit;
- secrets remain outside source control;
- failure of the persistence layer fails safely;
- tests cover session lifecycle and failure behavior;
- CI passes formatting, compiler checks, Clippy, tests and advisory audit with PostgreSQL available;
- a successful hosted deployment starts with the migration available inside the container.

### 3.2 Database foundation

**Implementation status: schema/migrations and PR #22 container correction merged; subsequent hosted activation observed. Managed-database durability, backup and restore remain unverified.**

PostgreSQL is introduced as a controlled platform capability, not as a reason to load sensitive company data immediately.

A temporary free Render PostgreSQL 18 instance is currently available for development verification and expires on **2026-10-30**. It is not permanent storage or backup infrastructure.

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

The order is intentional: **reliability and control precede sensitive information and feature scale.** The canonical-domain cutover is deliberately deferred and is not a blocker for the v0.3 engineering line.

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

### Current development checkpoint

Sol resumed development from Luna's PR #22 checkpoint on 2026-10-01 at Gyile's request. See `docs/governance/DEVELOPMENT_HANDOFF.md` for scope, attribution and remaining work.

PR #22 now also makes the production dependency graph reproducible and adds an automated production-container build/startup gate. The test uses a disposable PostgreSQL 18 database, verifies migration from an empty database, production refusal without `DATABASE_URL`, health/static assets, anonymous access controls and restart with persisted OAuth session metadata.

PR #22 and design PR #23 were merged after CI/Security. PR #24 added the member playground and was merged; its frontend was observed live after Gyile redeployed Render on 2026-10-02. Public health and anonymous access checks passed. Gyile's first real Gemini draft failed, so successful provider authoring and runtime Git publishing are still open gates; see [current evidence and recovery](MATH_PLAYGROUND.md).

The 2026-10-02 provider/documentation stopping scope ended when Gyile explicitly resumed development on 2026-10-03. PR #25 merged the safe provider diagnostics; PR #26 merged the Gemini 3 compatibility correction, each after exact-head CI/Security. The recommended private model setting is `gemini-3.5-flash-lite`; its configuration and real account access are not independently certified.

On resumption Gyile answered **“Not tested yet”** for the live draft retest. Sol therefore keeps AI generation and runtime Git publication pending and proceeds independently with the requested [calmer dark visual release](../architecture/MATH_PLAYGROUND_DESIGN.md#next-minor-release-calmer-dark-visuals), testable using synthetic demonstrations without a provider account.

PR #27 merged after exact-head CI/Security and screenshot review. It adds exhibit bundle 0.2.0, a dark renderer and panels, all three subdued palettes, explicit animation opt-in, and exact-byte preservation of older packages. The mathematical engine algorithms and formal reveal are unchanged; proofs remain deferred to capability 0.3.0. Browser CI records desktop/mobile screenshots for review. Automated contrast checks do not establish Gyile's individual eye comfort.

On 2026-10-03 Gyile reported manually deploying the latest commit and that the site is Live. Sol independently observed HTTP 200 health and live frontend/CSS bytes matching PR #27; the authenticated running-revision field was not inspected. Real AI generation and runtime Git publication still require the independent acceptance checks below. Sol prepared an original 1,477-byte metric-space `.tex` smoke test, successfully compiled locally; it is synthetic material, not coursework. The current renderer should use a mnemonic and the reveal must preserve the complete general definition.

Next actions, in order:

1. Confirm the running revision and assess the dark demonstrations/formal/source panels for Gyile's comfort.
2. Privately confirm the runbook's Gemini settings and make one small synthetic live draft. Record success or the exact error without repeated quota-consuming retries.
3. After mathematical review, verify one source-free runtime Git publication and saved-exhibit replay. Do not claim end-to-end activation before these pass.
4. In the next minor playground iteration, implement the [reusable authoring policy and controlled scope](../architecture/MATH_PLAYGROUND_DESIGN.md#next-minor-release-reusable-authoring-policy-and-controlled-scope): versioned automatic instructions, one complete concept, explicit selection/context-needed states and bounded large-input handling. This is planned; those result states are not supported by the current schema/UI.
5. Evaluate one relevant course concept for recall/usefulness. Continue proofs or new mathematical engines only when they support primary work; avoid duplicating LaTeX, Git or Obsidian workflows.

The canonical domain remains deferred. The wider company roadmap resumes section 3.3 (durable audit/event persistence) before activating any contract/engagement records; playground work does not satisfy those prerequisites.
