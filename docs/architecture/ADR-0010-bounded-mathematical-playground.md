# ADR-0010: Bounded mathematical playground authoring

**Status:** Accepted for the initial implementation; hosted acceptance remains a separate gate.
**Date:** 2026-10-01
**Collaborator:** Sol, continuing Luna's foundation at Gyile's instruction.

## Context

Gyile's LaTeX, Git repositories and Obsidian already preserve primary work. The playground adds a playful memory aid, not a replacement notebook or workflow tracker. The original [design](MATH_PLAYGROUND_DESIGN.md) calls for AI-informed, versioned mathematical engines, creative interaction and staged formal reveals.

Unrestricted AI-generated programs cannot be automatically trusted merely because they accompany mathematics. Running a general code-generation service would also require an isolated build/review system that this application does not currently have.

## Decision

Use the same Rust repository and existing member gateway. A dedicated `src/math_playground` domain owns intake, authoring, persistence and publishing. Pure JavaScript engines and a Canvas renderer live under `math-playground/`. Four Sol-reviewed, synthetic demonstration packages work without AI credentials.

AI receives the entire bounded selection through the explicitly configured provider: OpenAI Responses or Gemini `generateContent`, with a JSON schema and no tools. OpenAI requests use `store: false`; Gemini requests use its supported `responseMimeType` / `responseJsonSchema` fields and make no request-level retention opt-out claim. It extracts one concept, a verbatim source anchor, the general formal statement and a scene specification. The application independently validates field sizes, filename/source membership, the quote and allowlisted options. These checks establish provenance/shape, not mathematical truth.

A deterministic compiler creates a unique exhibit-specific engine module from reviewed code and allowlisted configuration. Its identifier and palette make the package distinct. Initial mathematical models cover Euclidean circle circumference, reciprocal-sequence convergence and finite permutations. Other concepts use an explicitly labelled mnemonic scene. This is constrained AI-assisted code generation, not a universal LaTeX-to-program compiler or arbitrary new AI-authored algorithms.

All executable output is reviewed application code. Uploaded text and AI prose never enter executable code. The first implementation therefore runs those modules in the existing trusted application context and keeps the existing CSP and frame protections. This deliberately replaces the proposal's sandboxed arbitrary-code player for this bounded stage. If arbitrary code generation is later introduced, the separate build/review system and isolated player are prerequisites, not optional extensions.

The exact accepted engine/renderer bytes are stored with a private exhibit and served from authenticated, owner-scoped endpoints. Publication commits those same bytes plus a source-free manifest into `math-playground-artifacts`, using a non-forced fast-forward update. It never modifies `main`. Raw course files, titles, excerpts, formal source statements, coursework hashes and member identity do not enter the public package. A repository snapshot is linked only after Git confirms the reference update. Retry checks existing manifest and executable bytes before accepting an existing package.

Exhibit engine capability is **0.1.0**; formal reveal capability is **0.2.0**. These are independent of application release numbering. The reveal contains notation/domains, hypotheses, the general statement, source anchor and mapping limitations. Theorems explicitly defer proof to **0.3.0**. AI formal statements remain visibly unverified until the member checks their source, assumptions and mapping and saves the exhibit. A review acknowledgement is not a machine-checked proof. Definitions do not acquire invented theorem proofs.

## Intake, attention and cost boundaries

- One selection: 1–8 UTF-8 `.tex` files, at most 32 KiB each and 64 KiB combined. The proposal's larger upload limits are reduced for this first authoring budget. Oversized selections are rejected; there is no silent context truncation.
- The user selects a self-contained concept and supplies relevant companion macros. No TeX execution, URL retrieval, shell execution or automatic filesystem include resolution occurs. Missing context must be disclosed by the authoring draft.
- Source text is held for the synchronous, bounded request and is not stored as a raw upload. The selected quote and concept derivative are stored privately. The input digest is SHA-256 of the server's canonical JSON serialization of the ordered filename/content selection, not a hash of the original upload container.
- Creation requires explicit consent for the selected provider. The site describes what text is sent and warns that Gemini unpaid services may use inputs/outputs for product improvement, including human review. The request's provider must match the current server provider; a changed provider requires new consent. Provider-side retention remains governed by the account's actual policy; OpenAI's `store: false` is not a zero-retention guarantee and is not sent to Gemini.
- Two authoring calls per instance; eight attempted AI requests per member per rolling 24 hours, reserved transactionally in PostgreSQL. Failed provider attempts count because they may incur cost. Each call has a 90-second upstream timeout; output limits are 4,000 tokens for OpenAI and 8,000 for Gemini (including its thinking budget). Both response envelopes are limited to 96 KiB and concept JSON to 32 KiB. No automatic retries, provider fallback or model calls during replay.
- This initial implementation uses synchronous requests, not the proposed durable background-job queue/cancellation API. Closing the browser is not a guarantee that an already-started provider request is cancelled. A queue becomes necessary before larger documents or longer-running authoring are supported.
- Private drafts become inaccessible after 24 hours. Expired rows are removed on startup and authenticated catalog access; this is not a scheduled physical-erasure guarantee. Saved private derivatives remain until member deletion. Minimal action/budget records retain no source text and are pruned after 30 days on the same cleanup paths.
- Publishing requires explicit mathematical-review and public-code acknowledgements plus a member/session request token. Download is available before publishing. Deleting a private exhibit does not delete public Git history.

## Consequences and evidence

No companion repository or new hosting service is required. Deployment needs existing PostgreSQL/OAuth plus the selected provider's optional key/model settings and an optional publishing token. Absent AI/publishing settings are honest unavailable states while demonstrations remain playable. Credential presence does not establish successful provider authentication or free-tier eligibility.

### Budget correction, 2026-10-02

Gyile reported no available money for API billing after creating an OpenAI key. Sol added explicit Gemini selection for a free-tier pilot, preserving the paid OpenAI option and all trusted-code/provenance boundaries. A free Google account/project remains a manual dependency. Its data-use terms differ from OpenAI and are disclosed before upload. Do not silently send course material to a different provider. Free quotas are neither unlimited availability nor a permanent service guarantee; project billing must remain disabled for the zero-spend pilot. The original source remains authoritative and the four demos remain useful without provider access.

The free development PostgreSQL environment's existing expiry and recovery limits still apply. Published code survives loss of that database in Git; private reveal/source mapping does not. This is low-risk study support, not an archive for irreplaceable materials or sensitive records.

Rust tests cover private routes, signed member sessions, request-token checks, source validation, provider-response handling and public-package redaction. PostgreSQL CI covers owner isolation, saved-byte recovery, retention and the authoring budget. Node tests cover circle scaling, reciprocal-tail bounds, exhaustive supported permutation enumeration and package digests. Browser CI exercises the actual frontend with clearly synthetic API fixtures. Container CI exercises migration packaging and startup. Real provider access, Git credential permissions, hosted source revision and live OAuth remain additional acceptance checks in the [playground runbook](../operations/MATH_PLAYGROUND.md).

Learning benefit is a hypothesis to test with Gyile's actual mathematics and recall. This release makes no claim of improved grades or income. Its practical test is whether a short interaction improves recall without distracting from the primary work.

### Provider compatibility correction, 2026-10-02

After activating Gemini, Gyile reported the generic provider failure. Sol corrected the Gemini `v1beta/models/{model}:generateContent` payload to use `generationConfig.responseMimeType="application/json"` and `responseJsonSchema`, matching [Google's published protocol](https://github.com/googleapis/googleapis/blob/master/google/ai/generativelanguage/v1beta/generative_service.proto). The previous Gemini-only `store` and `responseFormat` fields are absent from that protocol. OpenAI's request is unchanged. This compatibility correction does not establish which error the production account returned: its original upstream response was discarded.

A synthetic probe with an intentionally invalid, non-secret key independently confirmed that Google returns `API_KEY_INVALID` inside HTTP 400. The service now recognizes that structured reason, distinguishes request rejection, unavailable models, quota/access failures and timeout, and emits only fixed error codes plus HTTP status/provider metadata. Gemini error-body inspection is bounded to 16 KiB; upstream prose, keys and course text are neither returned nor logged. Failed attempts still count, and there is no automatic retry or fallback. Successful generation with Gyile's actual saved key remains a separate live acceptance gate.
