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
| `MATH_OPENAI_API_KEY` | A project API key from Gyile's OpenAI API account, authorized to create Responses. |
| `MATH_OPENAI_MODEL` | An exact model ID available to that account which supports Responses and strict JSON-schema text output. Set both AI values together. No model alias is silently chosen by the application. |
| `MATH_GITHUB_TOKEN` | A fine-grained GitHub access token restricted to `GyLiber/gyliber-command-center` with repository **Contents: read and write** and normal metadata read access. It must be permitted to create/update `math-playground-artifacts`. |

The existing GitHub OAuth token is used only for member authentication and is not reused for repository writes. A connected GitHub tool in a development conversation does not give a deployed server its own publishing token. Token expiry/revocation requires replacement in hosting settings.

Enable API billing/model access through the account if it is not already available. Configure an account/project spending limit appropriate for this pilot. The site's eight-request budget limits attempts, not money; monetary cost depends on the selected model and input size. This setup cannot be verified from credential names alone.

After saving settings, redeploy the verified `main` commit. Keep the linked deployment branch at `main`; do not deploy the artifact branch. If automatic deployment is disabled, use Render's manual deploy for the latest verified commit. Read the deployment log and verify health before testing the member flow.

Useful primary references:

- [OpenAI API keys](https://platform.openai.com/api-keys)
- [OpenAI Structured Outputs](https://developers.openai.com/api/docs/guides/structured-outputs)
- [GitHub fine-grained access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
- [GitHub Git trees and token permissions](https://docs.github.com/en/rest/git/trees)
- [Render deployment controls](https://render.com/docs/deploys)

## Live acceptance: complete this before claiming end-to-end activation

1. Record successful CI/Security checks for the exact implementation head. The Rust/PostgreSQL, production-container and playground/browser jobs must pass.
2. Confirm the Render deploy uses the intended merged source revision. `/api/health` verifies service health/version; authenticated `/api/state` supplies the running deployment commit. A healthy prior release is not proof that the playground deployed.
3. Sign in as an allowlisted GitHub member. Open the registered Mathematical Playground. Confirm anonymous page/API/code requests are redirected or rejected and a non-member cannot read an exhibit.
4. Play each demo, change a slider, pause motion and use the formal reveal. Verify the general theorem/definition is present, source/demo labels are honest, and theorem proof is explicitly deferred.
5. Choose a small, low-risk UTF-8 concept excerpt with its needed macros. Check provider consent and create one draft. The full selected text is sent to the configured AI provider; raw uploads are not saved as source files by this app.
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

## Current evidence

This runbook defines live gates; it does not certify credentials, deployment, a real course upload or a successful runtime publishing call before those actions are observed. Update the release/PR evidence with exact results as verification proceeds.
