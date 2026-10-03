# Mathematics playground: external authoring and reviewed playback

Implemented by **Sol**, continuing Luna's foundation, at Gyile's instruction on 2026-10-03. This is the first delivery of ADR-0011: external proposal authoring, developer review/integration, approved catalog and archive playback. The live site still uses member access. No arbitrary executable importer, theorem proof checker or automatic universal LaTeX converter is claimed.

## Gyile's two actions

1. Sign in, open **Command Center → Mathematical Playground**, and choose one complete concept's `.tex` files. Optionally name the concept in **Optional concept focus**. Click **Download authoring prompt**. This reads the selected files in the browser and downloads a text prompt. It uploads no source to the website and calls no AI. Give that prompt to an AI platform you already use, or attach the original `.tex` directly to Sol here. Share only material you have permission to share under that platform's terms.
2. Return the AI proposal and original source reference to Sol. Sol reviews the source mapping, mathematics, code and tests, records accepted files in Git, registers the catalog entry, passes CI/Security and deploys. If the AI says **NEEDS_CONTEXT** or **NEEDS_SELECTION**, provide the requested definitions or choose one concept first. An AI proposal is not yet an accepted exhibit.

The preparation limits are 1–8 distinct UTF-8 `.tex` files, at most 32 KiB each and 64 KiB combined. Empty files, invalid filenames/encoding and oversize inputs fail explicitly; nothing is silently truncated. LaTeX is never executed or followed to external includes. The optional hosted AI workflow remains in a collapsed **Optional hosted AI authoring** section; it is not part of this new path.

The original LaTeX remains in its primary home. The public repository cannot store private course excerpts, verbatim source quotations or member identity. Private source review material must stay outside public commits and requires its own approved durable home. The first release uses only Sol's original synthetic metric-space sample; it creates no new private source archive.

## Play and keep the metric pilot

Select **Metric couriers**. Three point selections choose a start x, via point y and finish z in `{1,3,4}`. Direct distance is `|x−z|`, detour is `|x−y| + |y−z|`. For a strict detour, set x=1, y=4, z=3: direct 2, detour 4. Repeated points and zero-length routes are valid. **Reset** restores x=1, y=3, z=4 and pauses motion. **Animate** is optional; hiding the page stops frames.

Open **Reveal the actual mathematics** to read the full general definition with nonnegativity, universal quantifiers, separation, symmetry and triangle inequality. This bounded finite illustration is not a proof on ℝ or a substitute for the definition.

**Approved archive** lists every accepted identity/version even when removed from current gallery navigation. **Stable replay link** selects a recorded release through `?exhibit=metric-couriers&version=0.1.0`. **Download archived package** downloads JSON containing every exact local file, tests, README and manifest, not merely an engine snippet. Invalid/unknown versions do not execute anything. The new catalog/playback endpoints do not access the private exhibit database or provider.

## Reproduce from a fresh checkout

No AI key, npm install or database is required for standalone replay:

```sh
git clone https://github.com/GyLiber/gyliber-command-center.git
cd gyliber-command-center
node --test math-playground/exhibits/metric-couriers/0.1.0/tests.mjs
python3 -m http.server 8000 --bind 127.0.0.1 --directory math-playground/exhibits/metric-couriers/0.1.0
```

Open `http://127.0.0.1:8000/viewer.html`. Stop the local server with Ctrl+C when finished. The engine, renderer, formal reveal and assets are local; no AI/API/CDN request occurs. To retain the exact source revision, checkout the merge commit recorded in the PR release evidence before running these commands. A package JSON is also a record of those same files; return it to Sol if you need extraction assistance. Member-site hosting/session dependencies are separate from standalone replay.

## Developer integration and acceptance

1. Review the proposal and source before executing code. Keep private mapping out of Git. Approve publication rights for any generic formal text. Use `sol_reviewed_public_synthetic` for original synthetic material or `developer_reviewed_public` after explicit source/mathematics/code/rights review. A manifest label alone is not approval: publication must pass the developer-reviewed PR gate. Keep private source quotes in the existing approved private primary home; add no derivative private store implicitly.
2. Promote a reviewed package to `math-playground/exhibits/<id>/<version>/`. Use plain local JavaScript modules; separate pure engine from rendering, supply formal data, standalone viewer/CSS, invariant tests and README. Include no remote assets, credentials, hidden storage or dynamic code execution. A passing keyword scan is not a security certificate.
3. Independently calculate SHA-256 hashes for every file other than the manifest. Record those in `manifest.json`, together with classification, runtime contract and review/publication evidence. Never take an AI's approval or hash claims on trust.
4. Add a member-visible release to `math-playground/catalog.json`. `in_gallery: true` adds a navigation button; false keeps archive access. Retain accepted records and directories. New bytes require a new version. Build-time embedding serves only the catalog's declared files; there is no filesystem traversal, remote Git fetch, URL import or executable upload endpoint.
5. Add appropriate mathematics/browser tests. Run `npm test` in `math-playground`, Rust formatting/check/Clippy/tests, production-container and Security checks. CI checks the PR base to reject edits to accepted version directories or removal of archive identities. Existing 0.1.0/0.2.0 demos remain byte-identical.
6. Merge the exact checked head. Deploy that commit on Render using **Manual Deploy → Deploy latest commit** if automatic deployment is off. No new environment settings or paid account is needed. Verify health and deployed frontend bytes; then sign in and confirm the metric interaction, reveal, prompt download and archive package. Compare live package file hashes to the Git manifest. Record the actual deployed revision; do not infer it from a successful merge.

Tests cover all nine finite distances and 27 triples, invalid inputs, exact package hashes, prompt source preservation, authorization, allowed/unknown routes, browser interaction, keyboard/mobile layout, dark contrast, motion/reset, archive-only replay and preparation with private shelf unavailable. These checks do not prove arbitrary mathematics, hosted uptime, personal eye comfort or income outcomes.

## Stopping point and subsequent work

The review/publication step is intentionally performed by Sol through normal repository development. Data-only recipe import for registered models is a later iteration after practical pilot feedback. General executable upload requires its own isolated staging/review design and is not quietly enabled here. Proof capability remains deferred. Provider recovery is optional and does not block playback.

Before processing many course files, Gyile should judge whether this one example actually aids recall. The temporary database expiry on 2026-10-30, durable audit work and the wider company roadmap remain separate open issues.

## Verified repository checkpoint and hosted handoff

[PR #30](https://github.com/GyLiber/gyliber-command-center/pull/30) merged as `1d790442ea8dcddfa036d666d85879fbf2c147ed` after exact-head verification of `8058d6a1f920c7e908cef3340e5c2d8219a16805`:

- [CI 37106568446](https://github.com/GyLiber/gyliber-command-center/actions/runs/37106568446): successful format/check/Clippy/audit, **42 Rust tests**, **10 Node mathematics/package/prompt tests**, **2 browser tests**, and production-container startup/migration/access checks.
- [Security 37106568444](https://github.com/GyLiber/gyliber-command-center/actions/runs/37106568444): successful secret scan and CodeQL Rust/JavaScript/Actions.
- A separate fresh checkout of the checked head passed all ten Node tests. The browser tests also exercised the standalone viewer without private shelf/provider use. Desktop/mobile screenshots were reviewed; personal eye comfort is still for Gyile to assess.

**Historical independent-verification limit.** The cloud browser reached Render's cold-start surface, then its URL policy blocked live inspection. Sol did not attempt a workaround, inspect credentials or infer deployment from the merge. No signed-in production metric/prompt/package check is claimed.

On resumption, Gyile's first manual step is to open the existing Render web service whose public address is `https://gyliber-command-center-1bym.onrender.com`, open **Deploys → Manual Deploy → Deploy latest commit**, and wait for **Live**. If the service already shows the current merged `main` commit as Live, no duplicate deployment is needed. No new environment settings are required. Record the deployed commit (the implementation merge above or a later main commit retaining it), then sign in and verify:

1. Metric couriers loads; x=1, y=4, z=3 gives direct 2 and detour 4.
2. The reveal shows the general definition with all three universally quantified axioms and nonnegativity.
3. Choosing the synthetic `.tex` and downloading the authoring prompt succeeds without using hosted AI.
4. The archive package download succeeds; the version remains addressable by the stable replay link.

If deployment or any check fails, report the failing step and visible error without sharing keys or database URLs. At Gyile's latest instruction, development may stop temporarily now, with these hosted checks explicitly pending until resumption. Sol's [repository pause checkpoint](../governance/DEVELOPMENT_HANDOFF.md#current-temporary-pause-checkpoint--2026-10-03) records the audited main revision and successful post-merge checks. Broader work remains deferred until Gyile resumes it.

## Release resumption and reported pilot acceptance — 2026-10-03

Gyile subsequently reported **all four hosted pilot checks passed** and authorized continued development. This supersedes the temporary pause above and establishes maintainer-reported functional acceptance of the pilot; it does not supply an exact deployed revision or independently prove hosting conditions. The same workflow forms the prepared **application v0.3.0** release. After its checked commit is merged and deployed, confirm health version 0.3.0 and the Live State deployment commit, then recheck the four actions. See [the release record and exact final steps](RELEASE_0_3_0.md). Exhibit packages/capability versions are independent; no accepted package bytes or deferred proof status change.

## Application v0.3.0 live acceptance — 2026-10-03

Gyile confirmed the final commit was Live and the playground checks passed, then explicitly confirmed health `status=ok`, health/Live State version **0.3.0** and deployment commit `ae80a6d04b0753dd4f6bd6a90f75dc003ec37d3d`. This closes the pending application release gate above through maintainer-reported evidence. See [the completed release record](RELEASE_0_3_0.md). Sol did not independently inspect the signed-in site. No new deployment or retest is required for this documentation-only update; comfort/recall benefit and future course-source review remain next-work evidence.
