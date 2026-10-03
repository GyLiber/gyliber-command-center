# ADR-0011: External authoring and reproducible mathematics exhibits

**Status:** Accepted planning direction at Gyile's instruction; not implemented.
**Date:** 2026-10-03
**Collaborator:** Sol, continuing the GyLiber engineering collaboration.
**Supersedes:** The next-iteration priority of improving embedded AI authoring in ADR-0010 and the PR #28 plan. Existing deployed behavior and its history remain intact.

## Context and fixed outcomes

Gyile's small metric-space upload returned `ai_provider_unavailable` after the confirmed dark frontend deployment. This establishes a failed request, not a proven upstream root cause or permanent provider unreliability. Gyile requests a different delivery design instead of continuing provider troubleshooting.

The fixed outcomes are:

1. Mathematics originates in supplied LaTeX `.tex` files.
2. The output is a simple, interactive visual with an honest relationship to that mathematics and a formal reveal.
3. Exact executable code is retained in a repository so hundreds of exhibits can be archived and reproduced without appearing in the current gallery.

External AI authoring is acceptable. A live AI connection inside the site is not required. Moving authoring outside does not eliminate AI interpretation errors or the external platform's availability/account terms; it removes that platform from the viewing and publication runtime.

## Decision

Prioritize **external authoring → reviewed repository package → curated site playback**. Keep member access and existing dark demonstrations. Do not remove the current provider integration in this documentation change or spend further attempts as an implicit dependency of the new path.

Gyile's two actions are:

1. Provide one self-contained `.tex` concept and the [standard authoring template](../operations/prompts/MATH_PLAYGROUND_EXTERNAL_AUTHORING.md) to an AI platform already accessible to him, including Sol in this conversation. Receive one exhibit proposal/package, or a clear selection/context request.
2. Submit that package for publication. **Initially return it to Sol for repository review and integration.** A future member Import action can stage it for the same review process. This is not an existing site capability, and uploading executable code must not execute it automatically.

The engineering process behind action 2 performs validation, source/mathematics review, isolated verification, repository recording, catalog registration, CI/Security and deployment. These are real steps; two user actions do not mean two unchecked machine actions. The site supplies a concrete outcome: needs context, needs selection, validation failed, awaiting review, accepted, or deployed. Future proof verification remains separately deferred.

### Language and package boundary

Use plain JavaScript ES modules (`.mjs`) with HTML Canvas or SVG and CSS. Pure engine code computes mathematical state; renderer code turns that state into the drawing. Avoid a new framework, CDN, build chain or provider call per exhibit.

The accepted public replay package contains exact engine/renderer bytes, allowed local assets, a viewer/example, mathematical invariant tests and a versioned manifest. The manifest records identifier, version, model/example/metaphor classification, file hashes, runtime contract and review status. Store formal/source review material separately when private. Hashes are calculated by the import/review tooling, not trusted merely because an AI supplied them.

Reuse trusted mathematical models and the dark renderer when appropriate. For an unsupported concept, Sol reviews/authors a new mathematical engine through ordinary repository development. A data-only recipe import using registered engines is a later low-friction path; it cannot silently accept arbitrary formulas, JavaScript strings or invented model identifiers as executable behavior.

### Review and executable trust

An AI-produced program is a proposal. Inspect its code and dependencies before running it. Verify that it does not introduce network requests, hidden storage, remote assets, dynamic code evaluation, source execution, credential access or uncontrolled animation. A keyword scan or passing test is not a security certificate.

Existing application playback imports reviewed application modules in the member page. **Do not reuse that boundary for arbitrary uploaded programs.** Initial publication promotes only reviewed code to application trust through a PR. General executable import requires a separately designed isolated preview/player, a bounded message contract and deny-by-default permissions. Its isolation must cover direct navigation as well as embedding; a sandbox with both scripts and same-origin permissions on a same-origin frame is not adequate. See [MDN's iframe guidance](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe).

Before generated tests/build commands run, use isolated ephemeral verification with no production credentials, no privileged publishing token and no persisted checkout credentials. Review changes to workflows/test configuration as code. Do not execute untrusted submissions in privileged `pull_request_target` jobs; see [GitHub's security guidance](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target). The reviewed publication/deployment stage is separate from submission execution.

### Repository, catalog and reproduction

Start in the existing repository; no new account, companion repository or paid service is required for the pilot. Use versioned directories such as `math-playground/exhibits/<id>/<version>/`. Separate staging from approved releases; accepted identity/version bytes must not be overwritten. Identical resubmissions can resolve to the existing release; changed bytes need a new version.

Add a validated catalog with identifiers, versions, approved file locations, visibility and current-gallery membership. Current demo routes and buttons name four exhibits explicitly. Merely committing another directory or writing to `math-playground-artifacts` does not make it discoverable by the live application. The existing artifact branch is an archive for database-backed exhibits, not an automatic deployed gallery.

Initially deploy approved packages/catalog with the application. The accepted code used on the site must match the archived hashes and recorded source revision. Do not evaluate arbitrary GitHub URLs or branch heads on each visit. Archived packages remain in Git and retain a stable identifier/version even if removed from current navigation. Load the chosen scene only; do not start hundreds of canvases or animation loops.

Include simple local replay instructions and needed local assets. A fresh checkout must reproduce an accepted package without an AI key or the temporary derivative database. Existing member-session/hosting dependencies still apply to the live site. Preserve independent Git backups; Git history alone is not an indestructibility guarantee.

The current repository is public. Raw course files, verbatim source quotes, private course/member metadata and secrets must not enter public packages. Preserve primary LaTeX in its existing home. A generic formal reveal may be published only after explicit rights/publication review; private source mapping needs a durable approved private home. The expiring PostgreSQL shelf is not the permanent source archive. Do not silently broaden ADR-0010's public-data boundary.

## First implementation milestones

1. **One real pilot before building a general importer.** Use the existing synthetic metric-space `.tex`. Represent its real-line example on the finite subset `{1,3,4}` with `d(x,y)=|x-y|`, interactive point-pair selection and a route through a third point. Goofy couriers/creatures can visualize direct and detour lengths computed by the engine. Show the general metric definition separately; this finite example does not prove it for arbitrary spaces. Keep the dark palette, keyboard/touch controls and opt-in motion.
2. Verify all nine ordered-pair distances and all 27 triangle triples for this integer-valued finite example, as well as selection boundaries, symmetry and separation. Preserve the distinction between these checks and a proof of the general source concept. Review formal/source fidelity, screenshots and no-network replay.
3. Commit the complete reviewed package, register it, deploy and confirm its live code matches the recorded hashes. Success requires a useful actual interaction, not just an archived prototype.
4. Generalize the approved catalog/archive and member handoff guidance. Preserve old versions and direct access to archived releases.
5. Only then add a bounded data-only import for registered models. Add general executable staging/isolation only if real authoring needs justify the larger engineering scope.

## Acceptance and limits

The pilot succeeds when Gyile can author/handoff without a deployed provider key, view a reviewed interactive metric example, reveal the complete formal concept, and reproduce exact archived code from a fresh checkout without AI or the private derivative database. Invalid/ambiguous proposals do not become playable or silently discard source. No paid dependency is activated implicitly.

This plan improves operational independence; it does not guarantee universal LaTeX understanding, automatic correctness, zero human review, hosted uptime or learning/income outcomes. Judge its practical value using one relevant concept before converting many files. The broader audit/contract roadmap and proof capability remain unchanged.
