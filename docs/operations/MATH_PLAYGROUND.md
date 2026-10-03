# Mathematical playground: activation and acceptance

**Implementation:** Sol's continuation of Luna's foundation, 2026-10-01.
**Member entry:** `/command/math-playground`, also registered in the Command Center.
**Deployed architecture:** [ADR-0010](../architecture/ADR-0010-bounded-mathematical-playground.md).
**Provider-independent workflow:** [Manual authoring and replay](MATH_PLAYGROUND_MANUAL_AUTHORING.md) is the new default. Browser-local prompt preparation and the reviewed metric example do not require provider setup.
**Design direction:** [ADR-0011](../architecture/ADR-0011-manual-math-authoring.md), external authoring and reviewed repository packages. Provider setup below describes the existing implementation; it is not a required dependency of the new pilot.

## What works without additional accounts

An allowlisted member can play the approved metric-space pilot and four preserved repository-backed demonstrations, change their parameters, pause motion and reveal formal mathematics. The source labels explicitly identify these as Sol-authored demonstrations, not Gyile's coursework. New 0.2.0 visual bundles always start paused; choose Animate to start motion. Their scenes, formal/source surfaces and controls use the dark site palette. A later reduced-motion preference pauses animation; hidden pages stop scheduling frames.

The initial authoring models are circle circumference, the sequence `1/n` converging to zero and finite permutations. Other mathematical concepts receive a labelled mnemonic scene. Formal definitions/theorems, hypotheses and notation are revealed separately from the drawing; theorem proofs remain deferred to exhibit capability 0.3.0.

## Manual environment activation

Keep the existing production settings from [DEPLOYMENT.md](DEPLOYMENT.md). The playground shares the existing PostgreSQL connection pool and runs `migrations/0002_math_playground.sql` on startup. Docker must package both migrations and the `math-playground/` source artifacts.

Set these values in the existing Render web service's private **Environment** settings, never in Git or a chat message:

| Setting | Purpose |
| --- | --- |
| `MATH_AI_PROVIDER` | `gemini` or `openai`. When omitted, the legacy OpenAI configuration is selected. No automatic provider fallback occurs. |
| `MATH_GEMINI_API_KEY` | A Gemini API key created privately in Google AI Studio; needed only when Gemini is selected. |
| `MATH_GEMINI_MODEL` | An exact compatible model identifier for new projects, currently `gemini-3.5-flash-lite`; set both Gemini values together. |
| `MATH_OPENAI_API_KEY` | A project API key from Gyile's OpenAI API account, authorized to create Responses. |
| `MATH_OPENAI_MODEL` | An exact model ID available to that account which supports Responses and strict JSON-schema text output. Set both AI values together. No model alias is silently chosen by the application. |
| `MATH_GITHUB_TOKEN` | A fine-grained GitHub access token restricted to `GyLiber/gyliber-command-center` with repository **Contents: read and write** and normal metadata read access. It must be permitted to create/update `math-playground-artifacts`. |

The existing GitHub OAuth token is used only for member authentication and is not reused for repository writes. A connected GitHub tool in a development conversation does not give a deployed server its own publishing token. Token expiry/revocation requires replacement in hosting settings.

OpenAI key creation does not establish funded model access. OpenAI lists the free tier as unsupported for `gpt-4.1-mini`. Do not require Gyile to buy API credits to play the reviewed demonstrations.

For the current zero-spend pilot, select Gemini and use a Google AI Studio project that remains on the **Free tier**, without enabling paid billing. Google currently recommends `gemini-3.5-flash-lite` for new projects and lists free input/output on its Free tier; South Africa is an available region. Gemini 2.5 access is now limited to users who actively used those models previously, so the earlier `gemini-2.5-flash` setup recommendation is superseded. Availability and account quotas must be checked in the account; free limits can change. The application cannot determine a project's billing tier from its API key. The site's eight-attempt budget is not a monetary cap. If the project is later upgraded, its requests can incur charges.

Google's unpaid services may use inputs/outputs to improve products, including human review. Use only non-sensitive material that the member has permission to share. Gemini is therefore an explicit alternative with provider-specific consent, not an invisible substitution for OpenAI. This member pilot is for Gyile's adult study use in an available region; public/expanded access requires reassessing provider eligibility and terms. Both providers receive `store: false`, which is not a zero-retention guarantee. Gemini's request logging setting does not override unpaid-service product-improvement/human-review terms.

### Browser-only Gemini setup

1. Open [Google AI Studio API keys](https://aistudio.google.com/api-keys), sign in with a Google account and read/accept the terms if appropriate.
2. New users may receive a default project/key. Otherwise create a key using a new personal project or an existing suitable project. Keep the project on **Free tier**; do not choose paid billing or an upgrade. If key creation asks for payment or account permissions are unclear, stop and report the visible options.
3. Copy the key directly into the Render web service's private Environment page as `MATH_GEMINI_API_KEY`. Also add `MATH_GEMINI_MODEL=gemini-3.5-flash-lite` and `MATH_AI_PROVIDER=gemini` together. Leave the GitHub token and existing authentication/database/session settings unchanged. Unselected OpenAI values are ignored.
4. After the relevant correction has passed CI/Security and merged, deploy the latest `main` revision. For saved environment changes use **Save, rebuild and deploy**; if no settings changed, use **Manual Deploy → Deploy latest commit**. Wait for the deployment to become **Live** before retrying.
5. On the member playground, confirm that the consent and status name **Google Gemini**. Create one small synthetic draft. If quota/access fails, report the message rather than repeatedly using up the eight-attempt budget. Do not send the key to Sol.

Configured credentials are labelled as configuration, not verified access. Quota/access failures have explicit messages; the service never retries automatically or switches to paid OpenAI.

After saving settings, redeploy the verified `main` commit. Keep the linked deployment branch at `main`; do not deploy the artifact branch. If automatic deployment is disabled, use Render's manual deploy for the latest verified commit. Read the deployment log and verify health before testing the member flow.

Useful primary references:

- [OpenAI API keys](https://platform.openai.com/api-keys)
- [OpenAI Structured Outputs](https://developers.openai.com/api/docs/guides/structured-outputs)
- [OpenAI GPT-4.1 mini access tiers](https://developers.openai.com/api/docs/models/gpt-4.1-mini)
- [Gemini pricing and free-tier availability](https://ai.google.dev/gemini-api/docs/pricing)
- [Gemini API keys](https://ai.google.dev/gemini-api/docs/api-key)
- [Gemini data-use terms](https://ai.google.dev/gemini-api/terms)
- [Gemini model access and deprecation notices](https://ai.google.dev/gemini-api/docs/deprecations)
- [Gemini 3 migration requirements](https://ai.google.dev/gemini-api/docs/generate-content/latest-model)
- [Gemini available regions](https://ai.google.dev/gemini-api/docs/available-regions)
- [GitHub fine-grained access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
- [GitHub Git trees and token permissions](https://docs.github.com/en/rest/git/trees)
- [Render deployment controls](https://render.com/docs/deploys)

## Live acceptance: complete this before claiming end-to-end activation

1. Record successful CI/Security checks for the exact implementation head. The Rust/PostgreSQL, production-container and playground/browser jobs must pass.
2. Confirm the Render deploy uses the intended merged source revision. `/api/health` verifies service health/version; authenticated `/api/state` supplies the running deployment commit. A healthy prior release is not proof that the playground deployed.
3. Sign in as an allowlisted GitHub member. Open the registered Mathematical Playground. Confirm anonymous page/API/code requests are redirected or rejected and a non-member cannot read an exhibit.
4. Play each demo, change a slider, pause motion and use the formal reveal. Verify the general theorem/definition is present, source/demo labels are honest, and theorem proof is explicitly deferred.
5. Choose a small, low-risk UTF-8 concept excerpt with its needed macros. Confirm the displayed provider and applicable data-use terms. Check provider consent and create one draft. The full selected text is sent to the configured AI provider; raw uploads are not saved as source files by this app. A changed provider requires fresh consent.
6. Confirm the selected source quote is verbatim, all hypotheses and quantifiers are preserved, and the scene's model actually matches the source. An exact quote alone does not prove a correct interpretation. If incorrect, delete the draft and submit a clearer excerpt; the first release has no formal-statement editor.
7. Verify the draft is labelled unverified, is private to the creating member, and shows its source and engine/renderer SHA-256 values. Download the exact code package and check it contains no course excerpt, course title or member identity.
8. After mathematical review, approve public source-free code publication and save to Git. Confirm the receipt's immutable commit contains `math-playground/exhibits/<id>/<version>/{engine.mjs,renderer.mjs,manifest.json}` (new bundles use `0.2.0`; earlier bundles retain `0.1.0`) on the artifact branch. Compare both code files' SHA-256 values with the private exhibit. Replay uses those exact stored bytes and makes no new AI call.
9. Restart/redeploy the service and confirm the member's saved exhibit is still readable and playable. This requires the same surviving PostgreSQL database and session key. Published Git code alone does not restore the private reveal/source mapping.
10. Delete a disposable private fixture. Confirm it disappears from the shelf while any published generic code remains in Git. Confirm budget/retention semantics in ADR-0010 are understood.

## Local and CI verification

```sh
cargo fmt --all -- --check
cargo check --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
cd math-playground
npm ci --ignore-scripts
npm audit --audit-level=high
npm test
npx playwright install --with-deps chromium
npm run test:browser
```

PostgreSQL integration tests need a disposable test `DATABASE_URL`; otherwise their bodies return without exercising a database. CI supplies PostgreSQL 18. Browser tests use synthetic API fixtures and no live provider/token. Never use production credentials/database URLs for these tests.

The production-container smoke script also checks the mathematics migration and anonymous code boundary. The original application-wide 64-KiB HTTP request limit remains; only playground routes use a 512-KiB JSON envelope, and decoded source text is independently limited to 64 KiB.

## Recovery and package use

The artifact branch is an append-only application convention, not an indestructibility guarantee. Repository administrators can delete it, and token permissions cannot be restricted to a directory by a fine-grained token alone. Maintain repository backups as already required by [BACKUP_AND_RECOVERY.md](BACKUP_AND_RECOVERY.md).

A downloaded `.json` packet contains `manifest` and exact `files` entries for the pure engine and renderer. Extract them unchanged. The engine is an ES module suitable for Node; the renderer's `mount(container, engineModule)` function needs a browser DOM/Canvas. A saved Git package has the same two modules as separate files. The public package intentionally excludes the private formal/source mapping; the original LaTeX remains the authoritative primary record.

Bundle 0.2.0 updates the renderer and leaves engine mathematics unchanged. The compatible manifest field `engine_version` identifies the entire executable bundle, including renderer. Saved 0.1.0 code remains exact. Opening an older exhibit shows a dark choice before drawing: **Dim original colours** filters its Canvas display, while **Show original (light colours)** explicitly displays the original palette. Downloads and public Git code preserve the original palette either way; the filter is not embedded in them. New demonstrations/drafts need no dim filter.

The existing development database expires on 2026-10-30 unless replaced. Preserve primary mathematics in its existing LaTeX/Git/Obsidian homes; do not treat this temporary derivative shelf as their archive.

## Provider failure recovery

The Gemini request uses `responseMimeType: "application/json"` and `responseJsonSchema` under `generationConfig`. It retains `store: false` and does not send `responseFormat`. Both output formats are recognized by Google's live REST discovery; this compatibility change alone cannot establish the original failure cause. OpenAI keeps its existing Responses request. Both remain bounded and use the same independent concept/source validation. Gemini requests omit `candidateCount`, which Google documents as unsupported in Gemini 3 and later. The response parser still requires exactly one candidate.

| Visible failure | Action |
| --- | --- |
| Google Gemini rejected the API key | In Google AI Studio, select the intended free-tier project and copy a valid key directly into Render's private `MATH_GEMINI_API_KEY`. Redeploy; never paste the key into chat or Git. |
| Provider rejected request configuration | Report the visible message and deployed commit. Sol can inspect the fixed `MATH_AI_UPSTREAM_REJECTED` status/reason in Render logs. Do not repeatedly retry or enable billing as a speculative fix. |
| Model was not found / operation unsupported | Check `MATH_GEMINI_MODEL=gemini-3.5-flash-lite` and the account's current model access; redeploy after a settings correction. |
| Key or model access refused | Check the selected project's API permissions, key restrictions and provider eligibility. |
| Quota reached | Keep paid billing disabled for the zero-spend pilot. Use demos and check the provider's free quota/reset time. |
| 90-second timeout / temporarily unavailable | Use demos; later try a smaller self-contained excerpt. No automatic retry occurred. |

After deploying the correction, refresh the member page, confirm **Google Gemini**, and submit one small non-sensitive synthetic `.tex` excerpt, such as `For every Euclidean circle of radius r > 0, its circumference is C = 2\pi r.` Confirm a draft appears and its formal statement/source anchor match. If it fails, record the exact new message; each failed attempt consumes the rolling daily budget. There is no need to enable OpenAI billing for this test.

The old generic message alone cannot identify the original upstream cause. The correction uses established output fields and fixes missing error classification; it does not certify the saved key, model access or free-tier availability without a successful live request.

## Current evidence, 2026-10-02

- PR #24 merged as `3e2c982e5ff9112b5f3b721d23aa80996813759f`. Its exact implementation head `2974c65dc3f764680f15261f9532e84a9d8c7c53` passed CI and Security, including 39 Rust tests with PostgreSQL, five Node invariant/package tests, the browser fixture and production-container checks.
- After Gyile's Render rebuild/deploy, public health returned HTTP 200, version `0.2.0`; the live playground frontend exactly matched that implementation. Anonymous playground-page access redirected to login and anonymous API access returned HTTP 401. Application version `0.2.0` does not mean the unreleased v0.3 development line has been tagged.
- Gyile reports working member login, saved Gemini settings and a 30-day GitHub publishing token. Sol has not inspected secret values. The first real generation reported a generic provider failure; successful hosted AI generation and runtime Git publication remain unverified.
- The compatibility correction adds request/schema and structured-error regression coverage and browser checks for distinct safe failure messages. Its exact-head check/merge evidence belongs to its corrective PR; live acceptance must be recorded after deployment and a successful real request.
- The provider correction contained no visual work and ended at Gyile’s stopping checkpoint. The explicitly resumed 2026-10-03 visual work is recorded below.


### Follow-up model-access failure, 2026-10-03 (South Africa)

After deploying the diagnostic correction, Gyile reported **configured model not found / operation unsupported**. This identifies an upstream HTTP 404; it does not prove which model/provider was configured or certify the key. Google's current model/deprecation pages limit Gemini 2.5 to previous active users and recommend 3.5 Flash-Lite or 3.8 Flash for new projects. The earlier new-account recommendation was therefore unsuitable and has been corrected to `gemini-3.5-flash-lite`, a supported structured-output model with listed free-tier input/output.

Sol removed the Gemini-3-unsupported `candidateCount` request field and added regression coverage for its absence, preserving the exactly-one-candidate response check, token/schema limits, explicit model selection and no fallback. Account-specific access and successful live drafting remain unverified.

After this follow-up has passed CI/Security and merged:

1. Open the existing Render service's **Environment** page and click **Edit**.
2. Confirm `MATH_AI_PROVIDER` is exactly `gemini` and set `MATH_GEMINI_MODEL` to exactly `gemini-3.5-flash-lite` (no quotes, spaces or `models/` prefix). Keep the existing privately saved Gemini key.
3. Choose **Save, rebuild and deploy**. Confirm the service deploys the latest `main` containing the candidate-count correction, then wait for **Live**.
4. Refresh `/command/math-playground`, confirm the consent names **Google Gemini**, and submit one small self-contained synthetic `.tex` excerpt. Record success or the exact new message. Keep the Google project on **Free tier** with paid billing disabled.
5. If HTTP 404 persists with the exact settings above, verify project-specific model access using Google's `models.list` and `supportedGenerationMethods` guidance; do not guess model identifiers or switch to a paid provider. A successful AI Studio chat alone does not certify API access for the saved project/key.

This follow-up changed no visual feature. The later resumed visual work proceeds independently with synthetic demos; provider acceptance remains pending.

## Resumed visual work and remaining acceptance, 2026-10-03

PR #25 merged the safe diagnostics and PR #26 merged the Gemini 3/model guidance correction after exact-head CI/Security. On resumption Gyile reported the live retest **not tested yet**. No successful live draft or runtime Git publication is claimed.

Sol’s dark bundle 0.2.0 includes all four demos and three palettes, dark formal/source/upload/action surfaces, paused-by-default motion and legacy package preservation. Browser CI exercises the real frontend with synthetic fixtures and records `dark-desktop.png` and `dark-mobile.png` in the seven-day `playground-visual-review` Actions artifact. Automated contrast/focus/mobile checks and screenshot review do not establish individual eye comfort.

After the visual PR has passed CI/Security and merged:

1. In the existing Render service, deploy the latest verified **main** commit. Wait for **Live** and check the authenticated deployment revision. Do not deploy `math-playground-artifacts`.
2. Open the member playground. Confirm **visual 0.2.0 · reveal 0.2.0**, a dark Canvas/formal/source area, and an **Animate** button before any motion. Try all four demonstrations and mobile/keyboard controls. Report any surface still too bright or uncomfortable.
3. If an older saved exhibit exists, confirm opening it initially displays the dark palette-choice screen. Choose the dim view if desired. Its download must retain its original 0.1.0 renderer bytes/digest.
4. Follow the private Gemini model-setting steps above and perform one synthetic draft. Record success or the exact error. Keep paid billing disabled; demonstrations need no provider.
5. Only after a correct mathematical review, complete the runbook’s source-free Git publication and saved replay checks. These are the remaining end-to-end activation gates.

The free PostgreSQL instance still expires on 2026-10-30. The publishing token was reported with a 30-day lifetime; check its actual expiry privately before testing. Neither temporary storage nor the playground replaces primary LaTeX/Git/Obsidian work.


## Metric-space smoke test and next authoring iteration, 2026-10-03

Gyile reports manually deploying latest main and that the service is Live. PR #27 merged as `cd17c60945057214c960aea00e17e57cdf228104`; its implementation head `20673119f749bd56af655b788c0e11ce72e94db5` passed CI `37094994366` and Security `37094994364`, including 41 Rust tests, five Node checks, browser coverage and the production-container gate. Sol reviewed the synthetic desktop/mobile screenshots. Sol independently observed HTTP 200 health and live frontend/CSS bytes matching PR #27; the authenticated running-revision field was not inspected. Personal comfort and real AI/Git operations remain separate checks.

Sol supplied `metric-spaces-test.tex`, an original 1,477-byte UTF-8 test, and verified successful LaTeX compilation locally. It contains one general metric-space definition, the usual real-line metric as an illustration, and explicit synthetic provenance. No account information or private course text is needed.

1. On the member playground, select this `.tex` file only. Confirm the consent names the intended provider and review its terms.
2. Check consent, then choose **Create a playful draft** once. If it fails, report the exact visible message rather than repeatedly retrying.
3. A successful result should be an unverified **mnemonic metaphor**: a general metric space is not one of the current computed scene models. Creativity may vary.
4. Reveal the mathematics. Check `d:X×X → [0,∞)`, all-point quantification, `d(x,y)=0` iff `x=y`, symmetry, and `d(x,z) ≤ d(x,y)+d(y,z)`. The numeric real-line example must not replace the definition. A picture does not prove these axioms or equip its drawn points with a metric.
5. Check that the source quote appears verbatim in the file. A correct quote alone does not establish mathematical fidelity. Downloading the exact source-free packet is enough for this initial smoke test; public publication is a separate explicit review/consent action.

Gyile also requested the [reusable authoring policy and controlled-scope plan](../architecture/MATH_PLAYGROUND_DESIGN.md#next-minor-release-reusable-authoring-policy-and-controlled-scope) for the next minor iteration. The existing automatic prompt already extracts one concept; candidate selection, context-needed states, prompt/model provenance and controlled document segmentation are planned improvements, not present functionality. Existing input limits remain enforced before AI.


## Reported metric-test failure and design change, 2026-10-03

Gyile's single synthetic test returned **The AI provider is temporarily unreachable or unavailable. No automatic retry occurred.** The safe error covers transport failures and otherwise unclassified upstream statuses; it does not prove a permanent outage or identify the root cause. Real provider drafting and runtime artifact publication remain unverified.

Gyile requests progress through external/manual authoring rather than continued provider troubleshooting. [ADR-0011](../architecture/ADR-0011-manual-math-authoring.md) and the [external authoring template](prompts/MATH_PLAYGROUND_EXTERNAL_AUTHORING.md) record that plan. Initially send the template and a complete `.tex` concept to an accessible AI, then return its proposal to Sol for review and repository integration. This is a proposed handoff, not a newly deployed upload/import feature.

Do not paste generated programs into the current `.tex` upload, developer console or member page. The existing application loads trusted reviewed modules; generic executable proposals need review before integration. Further provider attempts, model changes or billing activation are not requested by this plan. Demonstrations and accepted package replay should remain available independently of authoring-provider access.

## 2026-10-03 external-authoring implementation

Sol implemented ADR-0011's initial handoff and reviewed playback path. The member page prepares the standard prompt locally, offers the versioned metric-couriers example, retains archive identities outside gallery navigation and downloads complete replay packages. Packages are validated and embedded at build time; CI rejects overwriting accepted versions. The private shelf can fail without blocking the curated scene or local prompt. See [complete procedure and limits](MATH_PLAYGROUND_MANUAL_AUTHORING.md). Hosted deployment is a distinct acceptance step recorded after merge; a provider-success or automatic importer claim is not made.
