# Mathematical Experiment Bench — Design 1.0.0

**Design document version:** 1.0.0

**Status:** Design baseline requested by Gyile; implementation not started

**Date:** 2026-10-04 (Africa/Johannesburg)

**Author:** Sol, continuing Luna's foundation for Gyile / GyLiber

**Current application release:** v0.3.0; this document does not release application v1.0.0

**Home:** Member Mathematical Playground in the existing repository

## 1. Responsibility and reason to exist

Help Gyile answer a specific question arising in current mathematics work by changing a bounded mathematical rule, observing its consequences, and exporting a precise, reproducible result. The result remains useful even when the larger module or qualification is incomplete.

Gyile selected this candidate over examination rehearsal and public demonstration/service features because those alternatives did not currently justify their demand on his finite attention. Those alternatives are not included in the next implementation queue. Primary work and immediate livelihood matter more than site activity. This design makes no claim that experiments will improve marks or generate income; practical usefulness must be demonstrated on a real question.

The experience should feel like reaching for a familiar tool: open the relevant bench, change one thing, see what happened, keep the result, return to the primary work. No daily reporting obligation, streak, learning dashboard, onboarding course or extra note editor is required. Existing LaTeX/Overleaf, GitHub and Obsidian remain the primary writing, source-history and note tools.

The bench extends the existing playground's mathematical behavior. It is not a new site, AI provider integration or general computer algebra service. Reuse [ADR-0011's reviewed authoring and replay boundary](ADR-0011-manual-math-authoring.md) and the [playground design](MATH_PLAYGROUND_DESIGN.md).

## 2. Scope of the first delivery

Choose member access within the existing Mathematical Playground. A single **Experiment bench** entry opens one reviewed metric-space experiment, already initialized to a useful example. Proposed navigation is a bench selection within `/command/math-playground`; a separate route is unnecessary unless implementation evidence warrants it.

The first delivery supports exactly two registered rules on the fixed finite domain `X = {0,1,2}`:

| Rule ID | Rule | Purpose |
|---|---|---|
| `absolute-distance` | `d(x,y) = abs(x-y)` | Working baseline |
| `squared-distance` | `d(x,y) = (x-y)^2` | Demonstrate a triangle-inequality failure |

Controls select the rule and the points x, y, z from that domain. No arbitrary formula text, user JavaScript, remote module URL, continuous drag value or unsupported domain is accepted. Adding another rule or domain requires its own mathematical review and package version. Other modules may obtain a bench later when an actual task justifies one; not every concept needs an experiment or a drawing.

The first delivery includes deterministic evaluation, an accessible dark visual, finite axiom checks, a concrete failure witness, the full metric definition, reset and result export. It preserves old packages and optional hosted authoring unchanged. It does not introduce proof checking, automated grading, public auto-publication, cloud result storage, general executable import or the other proposed candidates.

## 3. User workflow and attention budget

1. **Open:** The existing member playground opens the bench with ordinary distance and x=0, y=1, z=2 selected. A useful baseline is visible without setup or a file upload.
2. **Change:** Choose squared distance or select points. Update values and the scene immediately, with no network call. State what changed and which property failed.
3. **Inspect when needed:** Expand the definition, all finite checks or the selected failure witness. Keep detailed provenance out of the ordinary controls.
4. **Keep:** One **Export result** action downloads one readable `.md` file with a single fenced JSON replay record at the end. The prose and data describe the same evaluated state. One file avoids multiple-download prompts, ZIP tooling and a second saving step; a browser download failure is visible and retryable.
5. **Return:** Gyile can insert the small result into existing primary work. There is no required journal entry or extra acknowledgement.

An ephemeral page session holds current selections. Reset restores the reviewed baseline. No hidden localStorage, analytics or database writes retain actions. An exported result is the intentional persistence boundary; closing the page before export loses the current selections. Make this fact clear near Export without nagging prompts.

For the first accepted default, aim for a usable result with one rule change and one export action, besides normal navigation. This is an interaction target, not a promise about study speed. Assess usefulness in Gyile's real work; do not replace that assessment with clicks, visit duration or animations watched.

## 4. Mathematical contract and honest conclusions

Reveal the general definition: for a nonempty set X, a metric is a map `d: X × X → [0,∞)` such that for every x, y, z in X:

- `d(x,y)=0` if and only if `x=y`;
- `d(x,y)=d(y,x)`;
- `d(x,z) ≤ d(x,y)+d(y,z)`.

Nonnegativity is required by the codomain. Display it as a separate check so the interface accounts for all four familiar metric conditions. Never present the selected three points as the full definition.

For each registered rule, check all nine ordered pairs for nonnegativity, separation and symmetry, and all 27 ordered triples for the triangle inequality, including repeated points. Complete enumeration with exact integer arithmetic can establish these conditions for this specific finite set. It does not establish a theorem on an infinite domain.

For the squared rule at x=0, y=1, z=2:

```text
direct = d(0,2) = 4
detour = d(0,1) + d(1,2) = 1 + 1 = 2
triangle condition would require 4 <= 2; it fails
```

The ordinary rule on the same triple gives direct 2 and detour 2. Nonnegativity, separation and symmetry hold for both rules on X; the squared rule fails the triangle condition. The same valid witness also refutes the squared rule as a metric on R because those three points belong to R and the same formula applies. This latter conclusion follows from the explicit witness and source/domain review, not from treating a finite search as a universal proof.

Use distinct result states:

| State | Permitted meaning |
|---|---|
| `finite_conditions_satisfied` | All defined conditions hold on the exact enumerated domain with the stated arithmetic |
| `counterexample_found` | A named condition fails on the supplied valid witness |
| `not_evaluated` | Missing/unsupported context or a rule outside the registry; no mathematical verdict |
| `invalid_input` | Selection or replay data violates the input contract |

Use **Conditions satisfied on X** and **Counterexample found** in the ordinary interface. Never show an unqualified “theorem proved” or “correct mathematics” badge. A witness disproves a universal condition; inability to find one does not establish a universal claim. For later numerical benches, introduce a separately reviewed arithmetic/tolerance contract and label inconclusive results; this exact-integer pilot needs no floating-point tolerance.

## 5. Drawing and accessibility

Reuse the site's calm dark palette, readable dim text and small playful couriers or shapes. No white background, flashing, bright full-screen success state or required animation. Motion starts paused, respects reduced-motion preferences, stops while hidden, and is optional. Changing a rule must still work with animation disabled.

Draw selected points and direct/via connections, labeling values computed by the engine. Pixel length, faces, movement and route shape are illustrative; they must not imply that squared distances are Euclidean lengths or that the picture proves an axiom. Mark the model/domain clearly. Color cannot be the only failure signal: include the property name, exact values and an inequality in text.

Provide labeled native rule/point controls, keyboard and touch operation, a text equivalent of the scene, restrained status announcements and visible focus. Preserve user selections when opening the formal reveal. Check desktop and small-screen layout without making the drawing an obstacle to the result.

## 6. LaTeX source and review workflow

LaTeX remains the input to authoring a bench. It is not uploaded again whenever Gyile uses an already accepted experiment. Start from the existing original [metric-space test source](../examples/metric-spaces-test.tex); derive the {0,1,2} experiment explicitly and record that it is an independently authored example, not the original metric-couriers package's {1,3,4} domain.

For actual course material, use the existing local prompt preparation and external authoring handoff to Sol. Review one self-contained concept with its definitions, domains, quantifiers and dependencies. Do not silently omit hypotheses, truncate large source or invent missing definitions. Return needs-selection or needs-context when appropriate. An AI-authored rule, visual or test is a proposal until mathematical, code and publication-rights review is complete.

Keep original course source in its existing primary home. Do not commit private course quotations, module/member metadata or source hashes merely to advertise provenance. Public packages contain original synthetic or explicitly approved generic mathematics and code. Private source mapping needs an approved durable private home before such mapping is retained by the system. No new private archive is authorized by this design.

## 7. Architecture and repository permanence

Use the existing repository, plain local JavaScript ES modules, HTML/CSS and Canvas or SVG. No new account, subscription, framework, live AI call, CDN or per-interaction server computation is required. The existing Rust application/session infrastructure continues to authorize member page and package access.

Create a new reviewed package, provisionally `math-playground/exhibits/metric-experiment-bench/0.1.0/`. Design document 1.0.0, initial package 0.1.0, current application 0.3.0 and formal reveal capability 0.2.0 are independent versions. A future application minor release must be chosen and verified during implementation; do not create a release tag or bump Cargo for this documentation change.

Package responsibilities:

- Pure engine: validate rule/points, evaluate distances, enumerate finite conditions and return deterministic witnesses in a documented order.
- Renderer: display the engine state, controls and accessible text; return a cleanup function, following the existing reviewed runtime contract.
- Formal material: complete metric definition, rule/domain mapping and limits of each conclusion.
- Result exporter/replayer: export data and readable results; re-evaluate validated records with the exact registered engine.
- Manifest, tests, viewer and README: record reviewed identities, hashes, replay instructions and mathematical/code/rights evidence.

Follow existing catalog approval, compile-time embedding, allowed-file paths, immutable version directories and archive/direct-version access. If a new runtime contract or additional file names are needed, amend validators and embedding deliberately with tests rather than mislabeling a package as compatible. Never edit metric-couriers/0.1.0 or the preserved demo versions to turn them into this bench.

Retain all exact engine, renderer, formal, exporter and replay bytes plus tests and allowed local assets in the versioned package. Retain archive access even if removed from the current gallery. A fresh checkout must replay the accepted bench without AI, npm installation, the private derivative database or external assets. Git preservation and user exports support recovery; they are not a claim that one repository or hosting provider is indestructible.

## 8. Result record and replay boundary

The fenced replay JSON inside the exported Markdown is data, never executable code. Its initial proposed schema is `metric-bench-result-v1`; implementation must finalize and test the exact schema before activation. Include:

- schema identifier, exhibit ID/version and digest of the exact packaged manifest;
- engine SHA-256 from that manifest and application/deployment revision when available;
- selected rule ID, fixed domain, point selections, exact direct/detour values;
- finite check coverage, per-condition outcome and a deterministic failing witness when present;
- conclusion scope, arithmetic contract and explicit limitations.

The readable note contains the question, rule/domain, values, conclusion, coverage and package identity; it must be immediately usable without understanding the schema. No required free-text note, account identifier, private source excerpt or module label. User edits and clock values are not evidence of correctness or trusted provenance.

Replay reads the file as bounded text, extracts exactly one explicitly delimited JSON block and never renders or executes Markdown/HTML. Reject missing, ambiguous or multiple record blocks. The readable prose is for convenience and is not trusted mathematical evidence. Replay checks schema, size, known ID/version/digest, point membership and allowed rule before evaluating. Cap the entire export at 16 KiB, reject extra record fields, duplicate JSON keys, non-finite/fractional values, malformed types and unknown versions explicitly. Select only locally approved archived packages; do not fetch code from a record's URL. Recompute outcomes from inputs and compare any stored claims, treating conflicts as invalid/tampered data rather than displaying them as verified results. Hash matching identifies recorded bytes; it does not authenticate the author of a user-supplied result.

First replay support is through the standalone reviewed viewer's **Open result file** action. It loads bounded data locally and never executes file content. The live member page initially needs only export and stable package access; adding live result import is a later usability decision, not a prerequisite or an arbitrary-code import permission.

Exporting externally means a deliberate local file output, not automatic disclosure to the public internet. The site does not upload outcomes to GitHub or store each point change. To publish a generic result later, return it to Sol for normal rights/mathematics/code review and a repository commit. Public sharing is a separate explicit action.

## 9. Security and failure behavior

Apply existing member authentication, response/CSP controls and deny-by-default package access. New APIs, if necessary, require the same authorization and bounded input handling. Personal experiments remain ephemeral or intentionally downloaded; they do not create database dependence, broaden company-data permissions or resolve the temporary PostgreSQL expiry.

Registered rules are reviewed functions, not formulas passed to eval, Function, a template engine or an unbounded symbolic interpreter. Render labels/results as text; do not interpret source or replay strings as HTML. Reject unknown IDs, missing dependencies, invalid selections, digest mismatches and malformed replay with a short actionable message, preserving valid current state where possible. Do not substitute another rule or silently round inputs.

Review generated code before isolated execution. No production credentials or privileged publishing token enters proposal verification. Network requests, hidden storage and remote assets are outside the package contract. Failed export must not lose the evaluated state or report that a file was saved. Existing provider failure must not block the bench. Source rights, private storage, operational audit/contract prerequisites and [recovery responsibilities](../operations/NEXT_DEVELOPMENT_STEPS.md) remain in force.

## 10. Acceptance and implementation sequence

| Gate | Required evidence |
|---|---|
| Mathematical behavior | Ordinary rule satisfies all finite conditions; squared rule fails triangle at (0,1,2) with 4 > 2; all nine pairs and 27 triples evaluated, repeated points and zero distances included |
| Input boundaries | Unknown rules, invalid/out-of-domain points and malformed replay rejected; no arbitrary executable path |
| Correct communication | General definition retained; finite success, counterexample and unsupported states distinguished; export states domain and limitations |
| Reproduction | Exact package hashes checked; result export and standalone reload yield the same recomputed state; edited stored conclusions and wrong package versions fail visibly |
| Existing behavior | Prior immutable packages, archive, member boundaries and optional authoring remain intact; fresh-checkout replay needs no provider/private database |
| Presentation | Dark desktop/mobile/keyboard/text operation; paused and reduced-motion behavior; no dependence on color or animation |
| Release | Exact-head CI/Security, reviewed screenshots, merge, deployment health/revision, signed-in bench/export checks and verified source tag for the selected application release |
| Usefulness | Gyile uses it on one real current-work question, keeps a usable result and reports whether it removed friction; synthetic tests alone do not establish this |

On a later explicit resumption:

1. Confirm the particular mathematical question Gyile needs answered and the existing pilot's eye comfort. If the proposed metric bench does not serve that question, revise the bounded example before coding; do not force adoption.
2. Review source/domain/rule specifications and output wording. Use original synthetic content until actual course rights/context review is complete.
3. Build the new immutable metric package, exact engine checks, renderer, result exporter and standalone data replayer; register it using the existing reviewed catalog.
4. Run affected mathematics/browser/replay/access/hash checks and existing CI/Security. Review readability and practical result usability, then merge only the checked head.
5. Deploy, verify live revision and member bench/export behavior, obtain usefulness feedback, document the actual result and publish/verify the chosen application minor tag. Stop expansion if it costs attention without answering the intended question; archive useful code rather than creating a feature obligation.

**Today's stopping point:** create this version 1.0.0 design, update Next Steps and the development handoff, verify and merge the documentation, then stop. No runtime implementation, deployment, account signup, other candidate development or new release tag is part of this work unit.
