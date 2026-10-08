# Resolution Control — Feature Design

**Repository:** `GyLiber/gyliber-command-center`
**Target release:** application **v0.4.0** (proposed)
**Feature status:** typed domain core implemented; persistence/API, member UI and hosted acceptance pending
**Design date:** 2026-10-08

**Continuity:** Luna proposed this design in commit `a434801235ce4a9f2addb178e95f86b7e3033ea8` on `design/resolution-control-2026-10-08`, directly from main `9e2eabf352c0f0c723eea46826129c5dbfc3fe92`. Sol continues integration at Gyile's request, preserving that commit and adding the implementation contract below. GitHub authorship remains the authenticated contributor identity. The design integration did not change runtime behavior. The first implementation packet below adds a pure domain core, not a live module or v0.4.0 release.

## 1. Problem

The Command Center needs a compact control surface for a live client-resolution problem containing many simultaneous obligations, hard deadlines, unstarted/blocked/partly verified work, uncertain assessment coverage, a requirement for earliest feasible finish, a deliberate safety-net buffer, and incoming obligations.

The feature must make this state **obvious without becoming another source of attention competition**.

It is not a generic calendar, conventional task manager, productivity scorecard, or replacement source-of-truth for underlying work.

## 2. Product Intent

Add a protected member module named **Resolution Control**.

Its primary question is:

> **What is the client's current resolution state, what is threatened, what has actually been externalized/verified, and what is the next correct executable action?**

The interface should preserve the Command Center requirement of useful workflow output per unit of human attention.

## 3. Core Model

A resolution owns commitments, their actions, unresolved threats and a private change history. Each commitment carries deadline/finish/buffer controls, scope coverage and evidence; an action points to the work artifact in its existing source tool.

### Resolution

One top-level problem being solved for a client.

Fields:

- `id`
- `title`
- `client_ref`
- `state`
- `objective`
- `consequence_class`
- `updated_at`

### Commitment

A dated or not-yet-dated obligation or outcome contributing to the resolution.

Fields:

- `id`
- `resolution_id`
- `area`
- `title`
- `kind`
- `deadline_at`
- `earliest_finish_target_at`
- `buffer_target`
- `status`
- `scope_confidence`
- `coverage_state`
- `next_action_id`
- `source_ref`

### Action

The smallest executable unit GyLiber intends to externalize.

Fields:

- `id`
- `commitment_id`
- `description`
- `expected_artifact`
- `verification_method`
- `status`
- `started_at`
- `completed_at`
- `artifact_ref`

### Unknown / threat

A known unresolved condition whose date, scope, dependency, or resolution path is not yet established.

It remains visible without fabricated values.

## 4. Attention Design

### Layer 1 — Current Action

One manually selected **Current Action** is visually dominant.

It answers:

- What do we do now?
- What artifact should exist when finished?
- How will it be verified?
- Where do we start it?

### Layer 2 — Obligation visibility

All other commitments remain visible in compact form with:

- deadline;
- earliest-finish target;
- buffer state;
- current state;
- scope/coverage state;
- next action;
- threat indicators.

This prevents every deadline becoming a simultaneous command.

### No opaque global priority score

Do not create a single numerical priority score.

Dates and control signals remain explicit. GyLiber selects the current action through the established decision process.

## 5. Earliest-Finish / Safety-Net Control

A deadline is not the preferred finish point.

Record:

- `deadline_at`
- `earliest_finish_target_at`
- `buffer_target`

Derived states:

- **buffer healthy**
- **buffer shrinking**
- **buffer exhausted**
- **late**
- **target unknown**

Do not manufacture estimates merely to populate fields.

## 6. Assessment Coverage Control

Assessment readiness must distinguish:

```
unknown scope
→ scope identified
→ material mapped
→ deployed
→ stress-tested
→ verified
```

The UI must expose:

- unknown scope;
- unmapped material;
- deployed-but-untested material;
- failed stress-tests;
- verified capability.

A percentage must never imply evidence that does not exist.

## 7. Additional Readiness Dimensions

Where applicable, a commitment can track:

- written execution;
- oral presentation;
- board/whiteboard reconstruction;
- justification/substantiation;
- research contribution;
- practical implementation;
- test-case verification.

Only applicable dimensions are displayed.

## 8. Incoming Work

New requirements are first-class records.

Support an **Unclassified / New** state so new obligations can be captured without prematurely inventing:

- deadline;
- priority;
- scope;
- dependency;
- estimate.

Once understood, the item becomes a commitment or is closed.

## 9. Change History and Client Report

Record meaningful transitions:

- commitment created;
- deadline changed;
- scope clarified;
- action started/completed;
- artifact linked;
- verification passed/failed;
- blocker added/removed;
- unknown resolved.

Generate the daily client-facing report from those actual changes:

```
actions completed
→ evidence now existing
→ newly discovered facts/failures
→ control changes
→ unresolved threats
→ next actions
```

This avoids introducing a second manual evidence log.

## 10. Private Pilot Instance

The real Gyile instance is maintained separately from this public repository.

The private instance includes:

- current academic/project deadlines;
- assessment and tutorial sequence;
- A2-readiness target;
- current backlog/readiness state;
- known project dependencies;
- research-paper uncertainty;
- livelihood-related obligations.

The private instance must support exact dates and current work-state data without exposing those records in public source control.

A separate private seed/export is the appropriate place for the concrete October–December 2026 pilot data.

## 11. Data Boundary

The repository is public.

Therefore:

- generic feature architecture belongs in source control;
- real client workload, academic status and private obligation records do not;
- actual records require authenticated, server-authorized owner access and the activation gates in section 16;
- the Command Center is a control view, not the sole source of truth;
- the current temporary PostgreSQL environment does not by itself justify treating the feature as an irreplaceable archive.

## 12. Integration With Existing Command Center

Use the existing architecture:

- register Resolution Control through the typed module registry;
- protected route such as `/command/resolution-control`;
- protected backend API;
- existing authenticated member boundary;
- existing module metadata and freshness semantics;
- existing calm, dark, high-signal visual language.

Do not introduce microservices.

## 13. v0.4.0 Scope

### Required

- Resolution Control module registration.
- Protected page and API.
- Resolution / commitment / action model.
- Deadline and earliest-finish target.
- Safety-buffer state.
- One Current Action focus.
- Compact all-commitments view.
- Unknown/new-work capture.
- Assessment scope/coverage states.
- Meaningful change history.
- Daily report generation from history.
- Automated backend and browser tests.

### Explicitly out of scope

- generic calendar replacement;
- opaque automatic prioritization;
- AI-generated scheduling;
- unsupported time estimates;
- public display of private client workload;
- storage of irreplaceable work artifacts in the module;
- generic habit tracking;
- gamification.

## 14. Acceptance

A v0.4.0 implementation is accepted when:

1. the module uses the member boundary and server-side owner/action authorization;
2. all commitments remain visible without all becoming visually dominant;
3. one current action can be selected and started;
4. deadline, earliest-finish target and buffer state are unambiguous;
5. assessment scope, mapping, deployment and stress-test state are separate;
6. unknown requirements can exist without fabricated metadata;
7. meaningful state changes generate durable history;
8. a daily client report can be generated from that history;
9. tests cover state transitions, authorization, buffer calculations and browser behavior;
10. no real private client-state data is committed to this public repository;
11. concurrent edits cannot silently overwrite work, mutations and history are atomic, and export/restore and retention are verified before private pilot activation;
12. Gyile demonstrates capture → select → start → externalize → verify → report on the deployed revision and confirms that the module removes friction.

## 15. Design Principle

> **Make every important obligation visible, make the next correct action easy to start, and create room before the deadline to discover and correct what was missed — without turning visibility itself into additional work.**

## 16. Implementation contract — Sol review, 2026-10-08

This section resolves the previously unspecified controls. It governs the first implementation; it does not certify controls that have not been built or tested.

### Ownership and minimal state

Use one owner per resolution in v0.4.0. Derive the owner from the authenticated server identity; never trust an owner supplied by the browser. All commitments, actions, evidence, history, reports and exports inherit that owner. Queries and writes must include the owner condition. Another member receives the same not-found response as for an absent record. Shared client access and delegation need a later explicit policy.

One manually selected Current Action exists per owner across resolutions. Selecting or starting an action validates its ownership and current state. Completion does not automatically choose the next action. An action cannot start without a concrete instruction, expected artifact and verification method; incomplete captures remain valid new items.

Default pilot classification is **Confidential**, reflecting private academic/work state. Minimize records to control metadata and references. Do not copy coursework, agreements, financial credentials, personnel records or irreplaceable work artifacts into this module. The original work stays in LaTeX, GitHub, Obsidian or another existing source. Reference links must be bounded HTTP(S) or plain text, never executable URLs or automatically fetched resources. Redact signed/private access tokens before saving. Public fixtures use original synthetic data only.

The existing auth policy is an authentication foundation, not resource authorization. The existing process-local `src/audit.rs` is not a durable history store. Reuse PostgreSQL/session infrastructure and the owner-filtering pattern in the playground, but add a dedicated versioned schema and tests. Do not inherit the playground's 24-hour draft expiry or reuse its AI budget events.

### Work and evidence semantics

Keep action lifecycle (`new`, `ready`, `in_progress`, `completed`, `blocked`, `cancelled`) separate from readiness evidence. A completed action means its output was recorded; it does not establish readiness or mathematical correctness.

Coverage is tracked per identified scope item and applicable dimension. Each item distinguishes scope identification, mapping, deployment, stress-test result and verification. A single successful example cannot certify an entire module. New/changed scope makes affected verification stale until reviewed again; a failed later test invalidates the affected readiness claim. Never hide unmapped scope behind a percentage.

A verification record needs scope/dimension, method, artifact reference, pass/fail outcome, actor, server timestamp and scope revision. Display it as a recorded verification by that actor, not machine-certified truth. Overall verified readiness requires identified scope, all required items/dimensions passing against the current scope revision, and no unresolved blocking threat. No automatic proof checker is introduced. Correcting evidence appends a superseding event rather than rewriting history. Reopening work clears its current verified-finish status while preserving the prior evidence.

### Exact time and buffer rules

Store known instants in UTC with an explicit input offset and display the selected timezone (initial pilot: `Africa/Johannesburg`). A date without a time is retained as a date-only source fact; it is not silently converted to midnight or used as an exact deadline. Unknowns are nullable, not zero or invented dates. The UI must show the precision and timezone.

Let D be the known hard deadline, E the preferred earliest-finish target, B the protected buffer duration (nonnegative whole minutes), and P = D − B the start of the protected window. Validate E ≤ P ≤ D. B is a chosen safety margin, not a forecast of remaining work. Reject inconsistent controls with a field-level explanation.

For unfinished work with complete controls, use these exact boundaries:

| State | Condition at authoritative server time t |
|---|---|
| Healthy | t < E |
| Shrinking | E ≤ t < P |
| Exhausted | P ≤ t < D |
| Late | D ≤ t |
| Target unknown | D, E or B lacks an exact value |

When E = P the shrinking interval is empty. When B = 0 the exhausted interval is empty. Show a separate known-deadline-reached flag even when other controls are unknown. Label the actual instants and remaining durations; the state describes the safety margin, not readiness or a predicted finish.

For currently verified work, freeze the outcome at the recorded verified-finish instant F: actual buffer is D − F; target buffer preserved means F ≤ P. Missing D/B yields an unknown buffer outcome. F > D remains visibly late; F = D means finished at the deadline with zero actual buffer. A later reopened/stale state resumes unfinished calculations, without erasing the historical outcome. Test all equalities, missing values, zero buffer, offset conversion and reopening with a controlled clock.

### Atomic history and safe mutation

Use bounded typed request bodies, field lengths and collection/page limits. Server-render or text-render user content; do not interpret HTML/Markdown scripts. Every mutation needs a session-bound CSRF request token, following the existing playground pattern, and authenticated owner policy. Keep requests same-origin; do not add credentialed cross-origin access. GET is read-only. Production database failure returns an unavailable state, never a misleading successful save or an in-memory fallback.

Update state and append its history event in one database transaction. Use a revision precondition so conflicting edits return a conflict for reload/review rather than last-write-wins. Retried requests must not duplicate actions or events; require an owner-scoped operation identifier with an atomic stored result. Events carry stable codes, actor, server timestamp, resource ID, revision and minimal changed values needed for the report. Payloads are private data and never go to ordinary tracing logs. Audit authorization denials using minimal operational metadata, without disclosing titles or private payloads.

Daily reporting is a private on-demand preview/download, not an automatic message to a client. Select a date and explicit timezone; derive its interval correctly from local day boundaries and a stable event cutoff. Separate recorded completions, linked evidence, passes/failures, control changes and unresolved threats as of that cutoff. List next actions from the same snapshot. An empty day reports no changes, not invented progress. Publishing or sending a report is a separate deliberate user action.

### Activation, retention and recovery

Build and test with synthetic records first. The temporary free Render PostgreSQL expires **2026-10-30**; it is not the durable pilot archive. Before real private onboarding, document and demonstrate durable hosting or an explicitly disposable pilot with private export/restore, owner isolation, durable mutation/denial audit, retention and deletion. Keep an independent private recovery copy. Export includes the versioned state, scope/evidence revisions and history required to reconstruct a report; restore validates a bounded data-only schema, assigns the authenticated owner and cannot inject identity or executable code. Test clean-database restoration and record what happens to sessions. A downloaded file without a restore test is insufficient evidence.

The pilot activation runbook must specify retention duration, owner-authorized deletion and backup expiry, distinguishing correction history from deletion of the whole private record. Application history is append-only during ordinary edits, not an exemption from the deletion policy. Backup restoration must not silently resurrect intentionally deleted records. Real pilot import remains manual and private; Luna's separate pilot document has not been supplied or reviewed in this packet.

These controls implement the relevant durable audit foundation for this bounded module. They do not activate Contracts & Engagements, complete the wider security roadmap or permit higher-classification data. If durable storage cannot be arranged without cost, report the concrete limit and keep synthetic operation available; do not imply that private operational readiness is complete.

## 17. Checkpointed delivery toward v0.4.0

1. **Recovery/design integration (this packet):** preserve Luna's commit, review this contract, update roadmap/handoff, pass repository checks and merge. Stop. No Render change, version bump or tag.
2. **Typed state and calculation packet:** bounded models, lifecycle/evidence transitions, deterministic clock/buffer calculation and targeted unit tests; update design from implementation evidence. Stop at a verified commit.
3. **Persistence/API packet:** migrations, authenticated owner authorization, transactional/idempotent history, conflict handling, private report/export/restore and integration tests. Synthetic data only until activation gates have evidence. Stop at a verified commit.
4. **Member workflow packet:** registry/page, calm dark keyboard/mobile controls, capture/current action/ledger/coverage/threats/report/export; browser tests for the complete synthetic workflow, loading/failure/conflict states and privacy. Stop at a verified commit.
5. **Hosted acceptance/release packet:** deploy the tested revision, verify migrations/session continuity and running identity; demonstrate end-to-end use and recovery, obtain Gyile's usefulness/eye-comfort feedback, document limitations and private activation status. Release application v0.4.0 only after its scoped gates pass; publish and verify the exact accepted tag. No tag is promised by this design.

After the typed-core packet passes its checks and merges, the next implementation packet is number 3. Mathematical Experiment Bench design 1.0.0 remains valid but is queued after this newly selected priority; it is not silently implemented alongside Resolution Control. Keep each packet reviewable and stop for Gyile's `continue` before starting another.


## 18. Typed-core implementation packet — 2026-10-08

Sol continues from main `4173ffcb43032019447d89952b427a080cad725c` on `feat/resolution-control-state`. The reusable library module `src/resolution_control/` owns bounded specification types, guarded action transitions, revision-aware scope/readiness, explicit time facts and deterministic buffer calculations. The binary's routes and registry are not changed in this packet. The application release remains v0.3.0; proposed v0.4.0 requires the later packets and live acceptance.

Implementation choices now explicit:

- Titles are limited to 160 UTF-8 bytes; notes and references to 2,048 bytes; identifiers to 64 ASCII letters/digits/hyphens/underscores. Blank and inappropriate control-character input is rejected. Scalar JSON deserialization applies those bounds; specification objects reject unknown fields. The API packet must also bound total request bodies and collections before domain construction.
- Web references permit HTTP(S), without embedded credentials, whitespace, query parameters or fragments. Non-clickable text references can identify existing work in other tools. Do not include secrets in either kind; there is no automatic fetching or execution. The initial stricter link policy avoids saving token-bearing links; a future extension needs explicit review.
- Time facts distinguish date-only source facts from exact, explicitly offset RFC3339 instants. Exact instants normalize to UTC and use whole-second precision with years 0001–9999. Subsecond instants with a nonzero fractional value are rejected, not silently rounded. A missing/date-only control is not an exact timestamp. Buffer duration is nullable, in whole minutes, capped at 525,600 (one 365-day year); zero is a deliberately selected zero margin. Arithmetic outside supported time bounds fails validation.
- Scope is capped at 128 unique item/dimension pairs. An identified scope must be nonempty; unknown scope is a separate valid state. No percentage is produced. Re-identification, mapping changes, re-deployment, newly added blocking threats and explicit readiness reopening conservatively invalidate earlier attestations by incrementing the scope revision. This initial conservative policy requires fresh tests for the whole current scope; narrower invalidation is deferred until its dependency rules are demonstrated.
- Stress-test and verification results are distinct. A new stress test, even a pass, supersedes the prior verification for that item. Passing verification requires a passing stress test on the current revision. All applicable items must pass, scope must be identified and blockers absent before a deliberate final confirmation mints a verified-finish value. An action completion cannot mint that value. A later failure clears current verified finish. Recorded evidence remains a human attestation, not a mathematical proof certificate.
- Domain aggregates serialize their latest state but do not deserialize browser-supplied status, actor, owner or verified-finish fields. The API packet must supply identity/time/revision from the server, enforce owner authorization and operation conflict/idempotency checks, and validate any trusted persisted/recovery state before reconstruction. It must transactionally archive superseded state/evidence: this pure core is not itself an audit store.
- Action capture may omit expected artifact/method, but becoming ready/startable requires both. Editing a not-yet-started plan returns it to new; ongoing work must be blocked before changing its plan. Completion requires an artifact reference. Reopening returns the action to new and clears current completion metadata; prior values must remain in private event history. Backwards clock updates and invalid transitions fail before changing state.

Focused tests cover specification/JSON bounds, safe references, explicit offsets/date-only preservation, incomplete and inconsistent controls, boundary equality/zero margins, a generated finite buffer-case sweep, action guards/atomic failures, scope and dimension coverage, stale evidence, blockers/failures/reopening and frozen finish outcomes. Required CI/Security evidence and exact head are recorded in the implementation PR before merge. No real pilot data or new service account is needed for this packet.

After this checkpoint, implement the dedicated PostgreSQL/API boundary, including transactional private history, authorization, revision conflicts/idempotency, exports, report cutoffs and validated restoration. Do not expose an apparently usable module that only keeps state in process memory. The 2026-10-30 database expiry remains an activation/release dependency.


## 19. Persistence/API implementation packet — 2026-10-08

Sol resumes from merged PR #42, main `9785e74c7b79b3c118d7177b7c7ebb570c31a4b1`, on `feat/resolution-control-api`. This packet implements the adapter promised by section 18; it does not add the member page or bump the application release. [The operations contract](../operations/RESOLUTION_CONTROL.md) is the detailed API, retention and recovery reference.

- One workspace belongs to the authenticated **stable numeric GitHub ID**, across all its resolutions. Existing sessions without that ID must authenticate again. No caller owner/actor/live timestamp/readiness status is accepted. A manually selected action is required to start; at most one action can run. Completion does not auto-select another.
- A bounded typed command log is the persisted authority. Transactional owner locks, expected revision and a rotating generation prevent silent overwrite. Owner-scoped operation IDs/hash/acknowledgements suppress duplicate retries. Replay uses the same guarded domain transitions, preserving superseded plans, scope/evidence and finish confirmations. PostgreSQL supplies live whole-second UTC time. Mutations, event and header update commit together; failure never falls back to memory.
- Workspace limits are 16 resolutions, 64 commitments, 128 actions, 128 threats, 2,048 events and 3 MiB command payloads. The earlier 128 scope-pair and scalar limits still apply. Regular bodies are 64 KiB; restore bodies 4 MiB including wrappers. History pages have at most 100 events. Capacity is explicit, with no silent deletion or compressed-away history.
- API reads require member identity/same-origin context; writes additionally need session CSRF. Failed authorization/mutation is minimally audited when storage is available. Administrative events/denials are minute-bucket counts without titles, payloads, IPs or user-agent strings. Command payloads remain private, never public Git history or trace output. A configured failing audit store returns unavailable.
- Daily JSON snapshots use an explicit fixed UTC offset (pilot +02:00), date boundaries and optional stable event cutoff. Named timezone/DST inference is not implemented. Reports reconstruct work/threats as of that snapshot and identify empty days/imported provenance. No automatic client delivery is introduced. The UI must render the control state and evidence categories clearly.
- Export schema 1 is bounded data-only command history with SHA-256 accidental-integrity checking. Restore requires empty work, explicit confirmation and the newest separate recovery metadata; it validates chronology/revisions/domain guards and assigns current ownership. Imported claims retain their supplied event times but all origins are marked imported; they are not independent proof of original identity/time or mathematical correctness.
- Purge deletes private event payloads, rotates generation and retains minimal deletion barriers/source roots so old backups and pending retries cannot silently revive work. Keep deletion metadata independently after every purge. A clean database cannot know a later deletion from an older workflow backup alone; newest-ledger supply is an explicit recovery responsibility, not an automatic/tamperproof guarantee. Checksums are not signatures. Idempotency/audit metadata has a 30-day opportunistic cleanup policy; deletion barriers remain while old backups may exist. The initial 4,096-marker/128-root bounds are explicit.
- The feature defaults **disabled** via `RESOLUTION_CONTROL_ENABLED=false`. Migration initializes from the existing PostgreSQL pool even while disabled; no paid dependency, separate repository or provider account is needed. Actual work/source artifacts stay in existing tools; references only are saved here.

Synthetic tests cover guarded aggregate/replay limits, private API authorization/CSRF/origin/error behavior, fixed-offset report boundaries/cutoffs, actual PostgreSQL isolation/reconnection/atomicity, concurrent stale edits, exact retries, export/import validation, purge receipts and clean-owner deletion barriers. Repository CI provides PostgreSQL 18; local database tests skip without `DATABASE_URL` and are not by themselves recovery evidence. Record exact-head CI/Security and production-container results in the implementation PR before merge, then stop.

Next on `continue`: member workflow packet with dark accessible capture/current action/ledger, all evidence stages/threats, private reports/recovery and synthetic browser acceptance. Hosted identity/session continuity, independent clean-database restore, storage expiry **2026-10-30**, deliberate activation and Gyile usefulness/eye-comfort feedback remain release/pilot gates. Proposed application v0.4.0 is not released by this backend checkpoint.


## 20. Smaller member preview packet — 2026-10-08

Gyile reasserted bounded batches after chat rendering/stream interruptions. Split packet 4 in section 17 into a **capture/current-action preview** followed by **evidence/report/recovery completion**. This changes delivery size, not v0.4.0 acceptance. The backend PR #43 checkpoint is main `b4567c14c615296a94c3e2f8a6a6cc6f9ff0e57a`.

The first member packet adds `/command/resolution-control`, protected by the existing page-member boundary, and a Confidential preview registry entry. The registry link is active only when the explicitly enabled feature has PostgreSQL and the session has stable GitHub identity; defaults remain reserved/disabled. Static files contain no instance data. API responses now identify the authenticated viewer so a refreshed account change clears earlier private drafts/displayed records.

The browser captures/refines resolution, commitment and action; retains optional unknowns; uses explicit date-only or exact whole-second UTC+02:00 controls; shows bounded ledger/current action and server-time buffer observations; and records references to existing output. It never computes readiness from completed actions. References become links only after the same restrictive HTTP(S) safety checks; other references and all private strings render as text. Input backgrounds and native control scheme are dark. No provider call, automatic public publication or browser storage of private records is added.

State is displayed only from a successful read. During loading, failed refresh or writes, mutation controls are disabled. An uncertain write retains its original operation ID/body in tab memory and offers an explicit retry; no automatic retry or new write is allowed meanwhile. Confirmed acknowledgement plus refresh completes the interaction. Server-accepted writes with failed refresh are identified separately. Rejected edits retain the draft; edited items keep their captured generation/revision through refresh and require deliberate reload/review after a conflict. Completion-reference drafts also survive refresh. Session loss/account changes clear private display/drafts; unsaved work is not persisted outside the tab.

Focused browser fixtures exercise capture → refine → select → ready → start → completed-output, text/XSS handling, keyboard/mobile/reduced-motion-compatible layout, date-only preservation, exact retries after simulated response loss, stale drafts after another tab's edit, and disabled storage/feature behavior. Rust checks separately verify the protected page, conditional module metadata and actual backend with PostgreSQL. CI screenshots are synthetic desktop/mobile review evidence, not private pilot acceptance.

Next packet finishes scope/dimension/evidence, threat, history/report and deliberate export/restore/delete controls and associated browser coverage. Keep `RESOLUTION_CONTROL_ENABLED=false` until later activation prerequisites are ready. This preview is not the v0.4.0 release and needs no manual Render action now.
