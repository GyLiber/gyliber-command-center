# Mathematical playground: activation and acceptance

**Implementation:** Sol's continuation of Luna's foundation, 2026-10-01.
**Member entry:** `/command/math-playground`, also registered in the Command Center.
**Architecture:** [ADR-0010](../architecture/ADR-0010-bounded-mathematical-playground.md).

## What works without additional accounts

An allowlisted member can play four repository-backed demonstrations, change their parameters, pause motion and reveal formal mathematics. The source labels explicitly identify these as Sol-authored demonstrations, not Gyile's coursework. Reduced-motion preferences default to a paused scene.

The initial authoring models are circle circumference, the sequence `1/n` converging to zero and finite permutations. Other mathematical concepts receive a labelled mnemonic scene. Formal definitions/theorems, hypotheses and notation are revealed separately from the drawing; theorem proofs remain deferred to exhibit capability 0.3.0.

## Manual environment activation

Keep the existing production settings from [DEPLOYMENT.md](DEPLOYMENT.md). The playground shares the existing PostgreSQL connection pool and runs `migrations/0002_math_playground.sql` on startup. Docker must package both migrations and the `math-playground/` source artifacts.

Set these values in the existing Render web service's private **Environment** settings, never in Git or a chat message:

| Setting | Purpose |
| --- | --- |
| `MATH_AI_PROVIDER` | `gemini` or `openai`. When omitted, the legacy OpenAI configuration is selected. No automatic provider fallback occurs. |
| `MATH_GEMINI_API_KEY` | A Gemini API key created privately in Google AI Studio; needed only when Gemini is selected. |
| `MATH_GEMINI_MODEL` | An exact compatible model identifier, initially `gemini-2.5-flash`; set both Gemini values together. |
| `MATH_OPENAI_API_KEY` | A project API key from Gyile's OpenAI API account, authorized to create Responses. |
| `MATH_OPENAI_MODEL` | An exact model ID available to that account which supports Responses and strict JSON-schema text output. Set both AI values together. No model alias is silently chosen by the application. |
| `MATH_GITHUB_TOKEN` | A fine-grained GitHub access token restricted to `GyLiber/gyliber-command-center` with repository **Contents: read and write** and normal metadata read access. It must be permitted to create/update `math-playground-artifacts`. |

The existing GitHub OAuth token is used only for member authentication and is not reused for repository writes. A connected GitHub tool in a development conversation does not give a deployed server its own publishing token. Token expiry/revocation requires replacement in hosting settings.

OpenAI key creation does not establish funded model access. OpenAI lists the free tier as unsupported for `gpt-4.1-mini`. Do not require Gyile to buy API credits to play the reviewed demonstrations.

For the current zero-spend pilot, select Gemini and use a Google AI Studio project that remains on the **Free tier**, without enabling paid billing. Google currently lists free input/output for `gemini-2.5-flash` and supports South Africa. Availability and account quotas must be checked in the account; free limits can change. The application cannot determine a project's billing tier from its API key. The site's eight-attempt budget is not a monetary cap. If the project is later upgraded, its requests can incur charges.

Google's unpaid services may use inputs/outputs to improve products, including human review. Use only non-sensitive material that the member has permission to share. Gemini is therefore an explicit alternative with provider-specific consent, not an invisible substitution for OpenAI. This member pilot is for Gyile's adult study use in an available region; public/expanded access requires reassessing provider eligibility and terms. Both providers receive `store: false`, which is not a zero-retention guarantee. Gemini's request logging setting does not override unpaid-service product-improvement/human-review terms.

### Browser-only Gemini setup

1. Open [Google AI Studio API keys](https://aistudio.google.com/api-keys), sign in with a Google account and read/accept the terms if appropriate.
2. New users may receive a default project/key. Otherwise create a key using a new personal project or an existing suitable project. Keep the project on **Free tier**; do not choose paid billing or an upgrade. If key creation asks for payment or account permissions are unclear, stop and report the visible options.
3. Copy the key directly into the Render web service's private Environment page as `MATH_GEMINI_API_KEY`. Also add `MATH_GEMINI_MODEL=gemini-2.5-flash` and `MATH_AI_PROVIDER=gemini` together. Leave the GitHub token and existing authentication/database/session settings unchanged. Unselected OpenAI values are ignored.
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
8. After mathematical review, approve public source-free code publication and save to Git. Confirm the receipt's immutable commit contains `math-playground/exhibits/<id>/0.1.0/{engine.mjs,renderer.mjs,manifest.json}` on the artifact branch. Compare both code files' SHA-256 values with the private exhibit. Replay uses those exact stored bytes and makes no new AI call.
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

The existing development database expires on 2026-10-30 unless replaced. Preserve primary mathematics in its existing LaTeX/Git/Obsidian homes; do not treat this temporary derivative shelf as their archive.

## Provider failure recovery

The Gemini request uses `responseMimeType: "application/json"` and `responseJsonSchema` under `generationConfig`. It retains `store: false` and does not send `responseFormat`. Both output formats are recognized by Google's live REST discovery; this compatibility change alone cannot establish the original failure cause. OpenAI keeps its existing Responses request. Both remain bounded and use the same independent concept/source validation.

| Visible failure | Action |
| --- | --- |
| Google Gemini rejected the API key | In Google AI Studio, select the intended free-tier project and copy a valid key directly into Render's private `MATH_GEMINI_API_KEY`. Redeploy; never paste the key into chat or Git. |
| Provider rejected request configuration | Report the visible message and deployed commit. Sol can inspect the fixed `MATH_AI_UPSTREAM_REJECTED` status/reason in Render logs. Do not repeatedly retry or enable billing as a speculative fix. |
| Model was not found / operation unsupported | Check `MATH_GEMINI_MODEL=gemini-2.5-flash` and the account's current model access; redeploy after a settings correction. |
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
- Next-session work is the [calmer dark visual plan](../architecture/MATH_PLAYGROUND_DESIGN.md#next-minor-release-calmer-dark-visuals). It is documented, not implemented in this correction. Stop feature development after this provider correction and documentation update, as Gyile requested.
