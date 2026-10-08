# Resolution Control — Feature Design

**Repository:** `GyLiber/gyliber-command-center`
**Target release:** application **v0.4.0** (proposed)
**Feature status:** reviewed design; implementation pending
**Design date:** 2026-10-08

**Continuity:** Luna proposed this design in commit `a434801235ce4a9f2addb178e95f86b7e3033ea8` on `design/resolution-control-2026-10-08`, directly from main `9e2eabf352c0f0c723eea46826129c5dbfc3fe92`. Sol continues integration at Gyile's request, preserving that commit and adding the implementation contract below. GitHub authorship remains the authenticated contributor identity. No runtime change or v0.4.0 release is implied.

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

The next implementation packet is number 2. Mathematical Experiment Bench design 1.0.0 remains valid but is queued after this newly selected priority; it is not silently implemented alongside Resolution Control. Keep each packet reviewable and stop for Gyile's `continue` before starting another.
