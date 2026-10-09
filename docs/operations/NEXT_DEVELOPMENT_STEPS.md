# GyLiber Command Center — Next Development Plan

**Document status:** Active roadmap  
**Planning date:** 2026-09-30  
**Last development update:** 2026-10-08
**Current release:** v0.3.0 Reviewed Mathematical Playground; maintainer-reported live acceptance
**Next development line:** Resolution Control, proposed application v0.4.0; member preview in smaller packets, followed by hosted acceptance
**Product principle:** maximize useful state and action per unit of human attention.

**Current stopping point:** Sol resumes from verified PR #43/main `b4567c14c615296a94c3e2f8a6a6cc6f9ff0e57a` on `feat/resolution-control-workflow`. This smaller packet implements protected capture/edit controls, obligation ledger, Current Action and explicit failure/retry/conflict behavior with tests. Stop after exact-head CI/Security, screenshot review and merge. Next on `continue`: evidence/threat/history/report and deliberate recovery controls, not deployment. The feature stays disabled; v0.3.0 remains accepted. Storage expiry on 2026-10-30 and private activation/recovery gates remain open.

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


## 11. Resolution Control recovery and next packets — 2026-10-08

Luna's branch `design/resolution-control-2026-10-08` added one design file in commit `a434801235ce4a9f2addb178e95f86b7e3033ea8`, directly from main `9e2eabf352c0f0c723eea46826129c5dbfc3fe92`. No application code, release tag or main-branch change occurred. No rollback is needed. Preserve the original branch/commit and integrate the reviewed design through `sol/resolution-control-integration` using the ordinary PR/check/merge process.

The [reviewed design, sections 16–17](../architecture/RESOLUTION_CONTROL_DESIGN.md#16-implementation-contract--sol-review-2026-10-08), defines owner authorization, completed-versus-verified evidence, exact buffer boundaries, transactional history/conflicts/idempotency, private report generation and activation/recovery gates. It supersedes any assumption that a member login alone or a temporary database makes the private pilot ready.

This takes priority over the previous bench queue at Gyile's latest instruction. Existing accepted v0.3.0 playground behavior and immutable packages remain the baseline. The bench is still a design, not discarded work. Resolution Control is proposed for **application v0.4.0**; this documentation integration is not that release.

| Packet | Reviewable outcome | Verification / stop boundary |
|---|---|---|
| Recovery/design integration | Luna provenance, reviewed contract, roadmap and handoff | Documentation consistency, CI/Security, merge; stop |
| Typed state/calculations (merged PR #42) | Bounded model, lifecycle, scope/evidence rules, precise buffers | Targeted unit tests including boundaries/missing times; CI/Security, commit/merge; stop |
| Persistence and private API (this packet) | Dedicated migrations, owner isolation, atomic/idempotent history, conflicts, report/export/restore | Synthetic PostgreSQL integration and authorization/failure tests, CI/Security; stop |
| Member workflow (next) | Dark focused action/ledger/unknowns/coverage and private report/recovery controls | Full synthetic browser workflow, keyboard/mobile/errors/privacy, CI/Security; stop |
| Hosted acceptance/release | Verified running revision and useful workflow; documented private activation status | Deployment/migration/session and restore evidence, Gyile feedback, scoped release/tag verification; stop |

Before private pilot onboarding, resolve the 2026-10-30 database expiry, demonstrate independent export restoration, durable owner/action audit and retention/deletion. These are concrete delivery gates, not a request to upload private data to this public repository. No new provider signup or paid AI is required by the design. If infrastructure needs a manual action, provide a specific runbook after the tested prerequisite work is complete.

Follow the checkpointed batch protocol: one coherent unit, ordinarily 1–4 commits, verify relevant gates, record branch/exact head/results/next unit, then stop. Gyile's `continue` starts the next packet. This replaces the older one-session progression for new development; historical checkpoints above retain their original meaning.


### Typed-core implementation checkpoint — 2026-10-08

The recovery/design packet merged through [PR #41](https://github.com/GyLiber/gyliber-command-center/pull/41) as `4173ffcb43032019447d89952b427a080cad725c`, preserving Luna's original commit after CI/Security success. Gyile then explicitly authorized end-to-end implementation under the batch protocol and designated Resolution Control as the next application minor release.

The current typed-core packet adds `src/lib.rs` and `src/resolution_control/` with bounded specs, guarded actions, scope/dimension/evidence transitions and precise controlled-clock schedules. See [design section 18](../architecture/RESOLUTION_CONTROL_DESIGN.md#18-typed-core-implementation-packet--2026-10-08) for concrete limits and conservative revision invalidation. Existing library dependencies are reused; time parsing/formatting features are enabled without changing the dependency versions. No routes, schema, registry entry or application version change yet.

The next packet is **persistence/API**, not another design or repeated core implementation: owner-filtered PostgreSQL schema, transactional private history, server identity/time, revision conflicts/idempotency, bounded report/export/restore and synthetic integration/authorization/failure tests. Before real pilot use, resolve database expiry and demonstrate recovery and retention/deletion. Ask Gyile for an explicit manual infrastructure action only when the tested prerequisite work exposes one; no manual action is required for the typed-core packet. Return an exact verified checkpoint and stop before advancing.


### Persistence/API implementation checkpoint — 2026-10-08

The typed-core packet merged through [PR #42](https://github.com/GyLiber/gyliber-command-center/pull/42) at `9785e74c7b79b3c118d7177b7c7ebb570c31a4b1`. Its exact tested head was `b89f185d80d770018bf37dea04ee4c75cac7565c`: [CI 37819305794](https://github.com/GyLiber/gyliber-command-center/actions/runs/37819305794) and [Security 37819305401](https://github.com/GyLiber/gyliber-command-center/actions/runs/37819305401) passed. Gyile's `continue` authorizes this distinct next packet.

This packet adds bounded command replay, migration 0003, stable GitHub-ID ownership, authenticated same-origin/CSRF endpoints, transactionally appended private history, revision/idempotency controls, private fixed-offset daily snapshots, checksummed data-only export/restore and deletion-generation barriers. See [design section 19](../architecture/RESOLUTION_CONTROL_DESIGN.md#19-persistenceapi-implementation-packet--2026-10-08) and [the operational contract](RESOLUTION_CONTROL.md). Synthetic PostgreSQL/HTTP tests exercise isolation, concurrency, failure, recovery and bounds. The implementation PR records exact-head repository gate evidence; a local run without a database is not persistence evidence.

Next on `continue`: the **member workflow packet** — register/protect the page; dark, keyboard/mobile capture and ledger/current action; complete scope/material/evidence/threat controls; same-snapshot private reports and deliberate export/recovery/delete; browser checks for the whole synthetic workflow and failures. Keep loading and uncertain saves explicit; refetch after acknowledgement; conflicts require review. Expose imported provenance and actual capacity limits. No automatic client messaging, scheduling, provider AI or executable import.

Keep feature activation disabled during backend preparation. No manual action is required from Gyile for this packet. Hosted acceptance follows the UI packet and must request concrete manual deployment/configuration actions only then. Resolve the 2026-10-30 storage expiry and prove a private independent backup plus latest deletion ledger can be restored before real onboarding. Application v0.4.0 and its tag await that end-to-end evidence. The existing locked `yoke-derive` 0.8.3 yank warning remains a separate dependency-maintenance item; this packet changes no dependency versions or audit policy.


### Smaller member-workflow packets — 2026-10-08

The API packet merged through [PR #43](https://github.com/GyLiber/gyliber-command-center/pull/43) at `b4567c14c615296a94c3e2f8a6a6cc6f9ff0e57a`, exact tested head `3ee25b248c3f3987f00410c9e99be8ecb1b91e22`. [CI 37829975848](https://github.com/GyLiber/gyliber-command-center/actions/runs/37829975848) and [Security 37829975775](https://github.com/GyLiber/gyliber-command-center/actions/runs/37829975775) passed, including 75 Rust tests with PostgreSQL 18, playground/browser and production-container checks.

At Gyile's renewed instruction about interface interruptions, split the larger member workflow into two reviewable packets. The current packet is a protected synthetic preview: resolution/commitment/action capture and guarded edit, exact/date-only/unknown controls, ledger and one manually selected Current Action, output references, visible server buffer observations, keyboard/mobile rendering and retained drafts. Uncertain saves offer only an explicit identical retry; stale editing drafts retain their original revision even after refresh. No optimistic saves, automatic action selection or embedded AI. The reserved Confidential registry entry becomes a preview link only when the feature/pool and current identity are available. Default activation remains disabled.

Next packet on `continue`: complete scope/dimension mapping/deployment, stress-test/verification/confirmation/reopening, threats, private history/daily report, export plus newest deletion ledger, bounded restore/purge controls and their synthetic browser scenarios. The protected preview is not yet the complete v0.4.0 workflow. Then a separate hosted acceptance/release packet verifies deployed identity, sessions, independent clean-database restoration, database expiry/disposable-pilot policy and Gyile's practical usefulness/comfort. Request manual actions only when those tested prerequisites are ready. No manual Render action is required now.

Browser tests exercise the real static files with a controlled synthetic API fixture. PostgreSQL/Rust tests independently verify the actual API and privacy boundary; do not describe the mock as hosted acceptance. Local Chromium download availability is an execution-environment constraint; exact-head CI provides browser execution and desktop/mobile review artifacts. Preserve GitHub commits/checkpoints after each packet; a chat rendering error is not evidence that repository changes were lost.

### Return plan after temporary stop — 2026-10-08

Gyile requested a temporary stop for today after documentation. The capture/Current Action preview is merged through [PR #44](https://github.com/GyLiber/gyliber-command-center/pull/44), main `2f3e5caecf15da55129ab7882902c5dd8a13409f`; tested head `e93d3370b1aed6632d08ea7b62d7f7a83b6fbaf1`. [CI 37835663960](https://github.com/GyLiber/gyliber-command-center/actions/runs/37835663960) and [Security 37835663898](https://github.com/GyLiber/gyliber-command-center/actions/runs/37835663898) passed. The merged/tested tree is identical. All 77 Rust, 12 Node and 4 browser tests passed; container/security gates and synthetic desktop/mobile review passed. The [handoff](../governance/DEVELOPMENT_HANDOFF.md#current-temporary-stop--2026-10-08) records the verification limits.

To keep the remaining interface work bounded, the previously planned evidence/report/recovery packet is now split at useful workflow boundaries:

| Next packet | Reviewable outcome | Verification and stop boundary |
| --- | --- | --- |
| A — Evidence, readiness and threats | Member controls for scope/dimensions, material mapping/deployment, stress-test/verification evidence, explicit readiness confirmation/reopening and blocking threats. Preserve unknowns and distinguish output completion from verified preparation. | Synthetic browser workflow through readiness and invalidation; real API tests where behavior changes; keyboard/mobile, retained drafts, conflicts/retries and privacy checks; appropriate CI/security gates; update documents, commit and stop. |
| B — Private reporting and recovery | History and daily report derived from server events; explicit private preview/download; export with the newest deletion ledger; bounded restore/purge with deliberate confirmations and generation-change handling. No automatic report publication. | Report boundaries and empty days, no duplicate history, uncertain operations, isolation and session-loss clearing; export/restore/deletion-barrier scenarios; review screenshots and appropriate CI/security results; update documents, commit and stop. |
| C — Hosted acceptance and v0.4.0 release | Demonstrate the complete synthetic workflow on the deployed revision, identity/session continuity, independent clean-database restoration with latest deletion ledger, and Gyile's usefulness/eye-comfort acceptance. Close the scoped release record and publish/verify its exact tag only after acceptance. | Request concrete manual deployment/configuration or acceptance actions from Gyile when their tested prerequisites are ready. Resolve temporary PostgreSQL expiry on 2026-10-30 and document durable storage or an explicitly disposable pilot before any real onboarding. Record evidence or a genuine blocker, then stop. |

On each `continue`, first inspect current main, open PRs, this handoff and the relevant source/design; choose one packet on a short-lived branch. Do not replay completed work or silently begin the following packet. If a packet grows beyond a coherent reviewable outcome, establish a tested sub-checkpoint and document its remainder. Record branch, exact tested head, merged main if applicable, checks and next unit in GitHub and return control to Gyile.

No manual action is required at this temporary stop. Keep `RESOLUTION_CONTROL_ENABLED` disabled and actual private records out of public fixtures. Application v0.3.0 and historical tags remain unchanged. Mathematical Experiment Bench remains queued after Resolution Control; unrelated dependency work is outside these packets. Documentation-only closeout requires no Render deployment or hosted retest.


### Packet A complete; next Packet B — 2026-10-08

[PR #46](https://github.com/GyLiber/gyliber-command-center/pull/46) merged the member evidence/readiness/threat interface at `e170e744bb6bdd8b2ba0532e5aa789c211661435`. Exact tested head `fa9524e65a444ab6e1207b0047bbc5ea7dfa9116` passed [CI 37843722795](https://github.com/GyLiber/gyliber-command-center/actions/runs/37843722795) and [Security 37843722800](https://github.com/GyLiber/gyliber-command-center/actions/runs/37843722800): 77 Rust, 14 Node, 6 browser tests and production/security gates. Browser tests use a controlled fixture; no hosted acceptance, private onboarding or personal comfort claim follows. The `playground-visual-review` artifact is archived in CI. Source packages and v0.3.0 release/tag are unchanged. Feature flag stays disabled.

**Next single bounded development packet (B):** Connect existing protected API endpoints to private paginated history, daily report preview/download with explicit date/UTC offset and cutoff, JSON export plus separately retrieved latest deletion-ledger metadata, deliberate owner-authorized purge and empty-workspace restore with confirmation. Show operation outcome uncertainty, imported provenance, rotation/generation conflicts, download custody and session-loss clearing. Test exact historical/empty-day reports, no duplicate events, tampered or stale restore/deletion ledgers and owner isolation with synthetic browser scenarios and existing PostgreSQL integration. Keep the newest deletion ledger separate from the backup so deleted data are not silently resurrected. No automatic publishing or private record fixtures. Stop after exact-head CI/Security, review and documentation; **Packet C** alone governs hosted acceptance, recovery demonstration, database-expiry resolution and v0.4.0 release/tag.


### Packet B — private reporting/recovery implementation, 2026-10-09

The branch `feat/resolution-private-report-recovery` implements the existing API's member-facing paged history, explicit-date/fixed-offset private daily report preview/download, paired export and newest independent deletion-ledger downloads, bounded empty-workspace restore and confirmed purge with retained latest receipt. Browser-side retries remain identical/idempotent; session loss clears private views. This is implementation awaiting exact-head CI/Security and merge, **not** hosted acceptance. See the [operations contract](RESOLUTION_CONTROL.md#member-packet-b--private-reports-and-recovery-controls-2026-10-09). No feature activation, new persistence, migrations or version/tag changes. Next only after this verified batch: Packet C host-side acceptance, actual private recovery evidence, storage-expiry decision and release gating. Stop rather than silently starting it.
