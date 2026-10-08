# Resolution Control — Feature Design

**Repository:** `GyLiber/gyliber-command-center`  
**Target release:** application **v0.4.0** (proposed)  
**Feature status:** design  
**Design date:** 2026-10-08

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

```
Resolution
  ├── Commitments / obligations
  │     ├── deadline
  │     ├── earliest-finish target
  │     ├── safety-buffer target
  │     ├── current state
  │     ├── scope/coverage state
  │     ├── evidence/verification state
  │     └── next executable action
  ├── Open threats / unknowns
  └── Change history
```

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

A dated obligation or outcome contributing to the resolution.

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
- actual records belong behind authenticated member access;
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

1. the module is protected by the existing member boundary;
2. all commitments remain visible without all becoming visually dominant;
3. one current action can be selected and started;
4. deadline, earliest-finish target and buffer state are unambiguous;
5. assessment scope, mapping, deployment and stress-test state are separate;
6. unknown requirements can exist without fabricated metadata;
7. meaningful state changes generate durable history;
8. a daily client report can be generated from that history;
9. tests cover state transitions, authorization, buffer calculations and browser behavior;
10. no real private client-state data is committed to this public repository.

## 15. Design Principle

> **Make every important obligation visible, make the next correct action easy to start, and create room before the deadline to discover and correct what was missed — without turning visibility itself into additional work.**
