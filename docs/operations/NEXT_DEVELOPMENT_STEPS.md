# GyLiber Command Center — Next Development Plan

**Document status:** Active roadmap  
**Planning date:** 2026-09-30  
**Last development update:** 2026-10-04
**Current release:** v0.3.0 Reviewed Mathematical Playground; maintainer-reported live acceptance
**Next development line:** Mathematical Experiment Bench design 1.0.0; implementation awaits explicit resumption
**Product principle:** maximize useful state and action per unit of human attention.

**Current stopping point:** Gyile selected the [Mathematical Experiment Bench design 1.0.0](../architecture/MATHEMATICAL_EXPERIMENT_BENCH_DESIGN.md) as the next candidate. Today is documentation only; stop after document verification/merge. No implementation or deployment is authorized by this planning work unit. Examination rehearsal and public demonstration/service candidates are not queued. Existing operational responsibilities, including temporary database expiry on 2026-10-30, remain open.

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

The live v0.3.0 application release includes the durable-session/container correction and reviewed mathematical playground. Gyile confirmed final deployment, all four playground checks, health status/version, Live State version and deployment commit; see [the release evidence](RELEASE_0_3_0.md). The current environment remains an engineering/demo environment. It is **not** an authorization to load critical company information.

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

`commit → CI → security → merge → deploy → health verification → release evidence → tag publication/verification`

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

### New direction after the metric-space request failed

Gyile's one synthetic metric-space attempt returned the safe `ai_provider_unavailable` message on 2026-10-03. The error does not identify its network/upstream cause. At Gyile's direction, further provider recovery is not a prerequisite for the next work unit. The three fixed outcomes remain LaTeX input, interactive mathematics and reproducible repository code.

Original delivery sequence (historical; implementation and the current pause/resumption checkpoint follow below):

1. Use the [external authoring template](prompts/MATH_PLAYGROUND_EXTERNAL_AUTHORING.md) and [ADR-0011](../architecture/ADR-0011-manual-math-authoring.md). Initially Gyile returns the generated proposal to Sol for review/integration; the site has no arbitrary-package Import action yet.
2. Deliver one reviewed interactive metric example from the synthetic LaTeX: the finite real-line subset `{1,3,4}`, direct/detour distances from `|x-y|`, and the full general metric definition in the reveal. Keep the existing dark/paused controls and classify the finite illustration accurately.
3. Preserve the complete accepted code/tests/manifest in Git, register it in an approved catalog, pass CI/Security, deploy and verify exact live bytes. A directory on an artifact branch alone is not a deployed gallery entry.
4. Verify no-AI replay from a fresh checkout without the private derivative database, mathematical/source fidelity, archived-version retention, keyboard/mobile behavior and Gyile's practical comfort/usefulness.
5. Generalize curated catalog/archive access. Add bounded data-only recipe import for registered models only after the pilot works. General executable uploads require separate isolation/review design; never run pasted programs in the signed-in member page.

The PR #28 prompt-control plan remains useful for source selection, missing context and output limits, but external authoring and publication now have priority. Existing hosted authoring/publishing acceptance is still incomplete; no new paid service or account is required for the pilot, and this planning change does not remove the deployed integration.

The canonical domain remains deferred. The wider company roadmap resumes section 3.3 (durable audit/event persistence) before activating any contract/engagement records; playground work does not satisfy those prerequisites.

### Implemented external-authoring checkpoint

On 2026-10-03 Sol completed the initial reviewed publication workflow in source: browser-local LaTeX prompt preparation; metric-couriers with deterministic distances and full general definition; immutable hashed replay package; build-time approved member catalog; archive-only/direct version access; full package download; standalone no-AI/no-database viewer and CI coverage. See [usage, acceptance and replay](MATH_PLAYGROUND_MANUAL_AUTHORING.md). The pilot uses original synthetic content. Other packages need explicit developer review and public rights approval. Existing hosted authoring remains optional.

### Temporary pause and ordered resumption — 2026-10-03

Gyile requested a repository review and documentation update before stopping temporarily. Sol audited `main` at `70a2177e952138f174a90dc6aee7c9253f6e22b4`: PR #30 implementation and PR #31 evidence are merged, no pull requests were open, and post-merge CI `37107340539` and Security `37107340552` succeeded. See the [current handoff](../governance/DEVELOPMENT_HANDOFF.md#current-temporary-pause-checkpoint--2026-10-03) for linked evidence. **Development may pause now. Hosted acceptance remains pending and may wait until resumption.**

1. Confirm the Render deployed revision includes PR #30; deploy latest main only if necessary. Complete and record the four [member-site acceptance checks](MATH_PLAYGROUND_MANUAL_AUTHORING.md#verified-repository-checkpoint-and-hosted-handoff). Fix a concrete failure before expanding the feature.
2. Ask Gyile to judge the pilot's eye comfort and recall benefit. Record the result; automated tests cannot establish personal usefulness.
3. Review one actual course-source proposal, including source rights, complete-concept selection and mathematical fidelity, before converting many concepts or extending storage/automation. Consider bounded data-only recipes after that evidence.

The existing catalog, archived replay, prompt preparation, formal definition and fresh-checkout tests are completed work; do not repeat their initial implementation. Arbitrary executable import, proof checking, provider retesting and the wider audit roadmap remain deferred. The temporary database expiry on 2026-10-30 still applies; this pause does not establish durable hosting or backup readiness.

### Resumed release completion — 2026-10-03

Gyile explicitly resumed development and asked whether the completed playground could constitute the next minor site release. Sol adopts **application v0.3.0 — Reviewed Mathematical Playground** as the bounded release, retaining all existing security gates. This supersedes the temporary pause above. It does not claim all operational-hardening roadmap items or exhibit proof capability are complete.

Gyile answered **“All four passed”** for signed-in metric/definition/prompt/archive checks. Record this as maintainer-reported pilot acceptance, without inventing a deployed commit. Sol prepares the shared Cargo-driven release identity, current README/changelog and [v0.3.0 release record](RELEASE_0_3_0.md), then runs CI/Security and merges after success.

Next actions are: deploy the tested release revision if necessary; record health version 0.3.0 and Live State deployment commit; confirm the four pilot checks still pass; update the release record with that evidence. Then seek personal comfort/recall feedback and one permitted course-source proposal. Further feature work follows that evidence, rather than reopening the initial implementation. The wider audit/session-continuity/backup/contract gates and 2026-10-30 database expiry remain open.

### v0.3.0 live acceptance completed — 2026-10-03

Gyile reported the final commit deployed Live and playground tests passed, then explicitly confirmed health `status=ok`, health/Live State version **0.3.0**, and deployment commit `ae80a6d04b0753dd4f6bd6a90f75dc003ec37d3d`. Sol recorded this as maintainer-reported end-to-end acceptance in [the release record](RELEASE_0_3_0.md); no independent signed-in production observation is claimed. This supersedes the pending release gates above. The documentation-only acceptance record needs no deployment/retest.

The scoped application minor release is complete. Next development waits for Gyile's direction and evidence of eye comfort/recall benefit, then one permitted actual course-source proposal. Sol published and verified the application v0.1.0, v0.2.0 and v0.3.0 tags/release listings on 2026-10-04; see [tag provenance and future release rules](RELEASE_TAGS.md). Wider operational hardening and deferred exhibit proofs remain open. Future release closure includes publication and verification of its exact source tag.


### Mathematical Experiment Bench selected; documentation stop — 2026-10-04

Gyile prioritizes a useful tool for current mathematics work and immediate livelihood over more features competing for his attention. Sol records [design document version 1.0.0](../architecture/MATHEMATICAL_EXPERIMENT_BENCH_DESIGN.md). This is a design version, not application v1.0.0, an implemented feature or a new release. Application v0.3.0 and its published tags remain the accepted baseline.

The first proposed member bench compares ordinary and squared distance on X={0,1,2}. It shows exact finite axiom checks and the triangle counterexample 4 > 2, with a calm dark drawing, complete definition and one-click readable result/replay export. LaTeX remains the authoring source; already reviewed benches need no repeat upload or live AI request. Use existing catalog/archive and normal reviewed Git publication, preserving prior immutable packages. Personal results are ephemeral until deliberately downloaded; public publication is a separate reviewed action.

After Gyile explicitly resumes implementation:

1. Confirm one actual current-work question and eye comfort; adjust the bounded example if it does not help that work.
2. Review mathematical/source/rights specifications, using original synthetic material until course-source review is complete.
3. Implement a new metric package provisionally at `metric-experiment-bench/0.1.0`, deterministic engine, dark accessible controls, export and standalone bounded-data replay. No arbitrary formulas, code uploads, new account or provider dependency.
4. Verify mathematics, witness scope, malformed inputs/replay, exact hashes, unchanged old archives, keyboard/mobile/motion behavior and required CI/Security before merge.
5. Deploy and verify the actual running revision and bench/export workflow, assess usefulness, update evidence and publish/verify the chosen application minor tag. Stop expansion if the bench adds friction without answering the intended question.

Examination rehearsal, public demonstration/service features, proof automation and general imports are not dependencies or approved additions to this queue. No learning, income or wider security-roadmap completion is inferred from the design. Finish today's documentation and stop; no signup, Render deployment or fresh release tag is required.
