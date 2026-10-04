# Application v0.3.0 — Reviewed Mathematical Playground

Prepared by **Sol**, continuing Luna's foundation, at Gyile's instruction on 2026-10-03.

**Status: v0.3.0 demonstrated end to end through maintainer-reported live acceptance on 2026-10-03.** This development is the next minor application release. It does not require completion of every item in the longer-term roadmap. A successful build alone is not an end-to-end release demonstration.

## Scope and versions

The release adds useful, provider-independent playback to the existing member hub: browser-local LaTeX prompt preparation, an external AI proposal/review handoff, the reviewed metric-space pilot, complete general definition, dark opt-in animation, approved catalog, immutable archives, package downloads and standalone replay. The ordinary PR/review boundary remains responsible for accepting source and code. Member hosting still needs the existing OAuth/session/PostgreSQL infrastructure; standalone replay does not.

| Version | Meaning in this release |
|---|---|
| Application 0.3.0 | Cargo package, health, Live State and site release identity |
| Metric couriers package 0.1.0 | Exact archived pilot files; remains immutable |
| Legacy dark bundle 0.2.0 | Preserved demo/renderer versions |
| Formal reveal capability 0.2.0 | General formal statement/definition; animation is not proof |
| Exhibit proof capability 0.3.0 | Deferred; not delivered by application 0.3.0 |

This release does not claim arbitrary executable import, automatic universal TeX conversion, successful optional hosted AI publishing, permanent database storage, completed operational audit/contract gates or qualification/income outcomes. Its practical recall and eye-comfort benefit still needs Gyile's assessment.

## Acceptance evidence

| Gate | Evidence / current status |
|---|---|
| Pilot implementation | [PR #30](https://github.com/GyLiber/gyliber-command-center/pull/30), merge `1d790442ea8dcddfa036d666d85879fbf2c147ed`; checked head `8058d6a1f920c7e908cef3340e5c2d8219a16805` |
| Pilot automated checks | [CI 37106568446](https://github.com/GyLiber/gyliber-command-center/actions/runs/37106568446) and [Security 37106568444](https://github.com/GyLiber/gyliber-command-center/actions/runs/37106568444) succeeded: 42 Rust, ten Node and two browser tests, production-container checks and security scans |
| Reproducibility | Separate fresh-checkout Node checks passed; standalone viewer and desktop/mobile screenshots reviewed in the implementation checkpoint |
| Hosted pilot behavior | Gyile initially reported **“All four passed”**, then confirmed the final commit was live and the playground tests still passed. Signed-in metric/definition/prompt/archive checks accepted on 2026-10-03; maintainer-reported evidence |
| Release identity change | [PR #33](https://github.com/GyLiber/gyliber-command-center/pull/33) merged as `f35f465db30c19f3975a7329f37267fe5861929d`; checked head `e8c87a96036fc9e90877b5b0c2913eaec70da61c`. Shared Cargo-derived page labels and package/lockfile version 0.3.0 |
| Release automated checks | [CI 37113351022](https://github.com/GyLiber/gyliber-command-center/actions/runs/37113351022) and [Security 37113350986](https://github.com/GyLiber/gyliber-command-center/actions/runs/37113350986) succeeded for that checked head: format/check/Clippy/advisory audit, 42 Rust, ten Node and two browser tests, production-container startup/migration/access/restart, secret scan and CodeQL Rust/JavaScript/Actions |
| Hosted application 0.3.0 identity | Gyile explicitly confirmed **“Yes, all match”**: health `status=ok`, health/Live State version `0.3.0`, and deployment commit `ae80a6d04b0753dd4f6bd6a90f75dc003ec37d3d` (PR #34 merge, retaining PR #33 runtime code). Maintainer-reported acceptance on 2026-10-03 |
| GitHub version tag/release | [v0.3.0](https://github.com/GyLiber/gyliber-command-center/releases/tag/v0.3.0) published on 2026-10-04 at accepted commit `ae80a6d04b0753dd4f6bd6a90f75dc003ec37d3d`; remote lightweight tag verified; Latest release |

Sol records Gyile’s explicit observations rather than independent browser verification. The prior cloud-browser URL-policy block remains; no bypass or inspection of secrets occurred. The [manual-authoring runbook](MATH_PLAYGROUND_MANUAL_AUTHORING.md) retains the functional test and replay details.

## Live acceptance recorded — 2026-10-03

Gyile reported **“Final commit deployed live and the playground tests pass.”** Sol then asked specifically about health status, both displayed release versions and the deployed commit; Gyile confirmed all matched the values above. This closes the scoped application v0.3.0 live-release gate. No repeat deployment or playground retest is required for this documentation-only record. The separate GitHub tag/release publication was completed on 2026-10-04; see [the verified tag history](RELEASE_TAGS.md).

## Acceptance procedure retained for future deployments

1. Open the existing Render web service for [the live site](https://gyliber-command-center-1bym.onrender.com). In **Deploys**, compare the deployed commit with release merge `f35f465db30c19f3975a7329f37267fe5861929d` or a later reviewed main commit retaining it. If needed choose **Manual Deploy → Deploy latest commit**, then wait for **Live**. No new account, environment variable or paid API is required.
2. Open [health](https://gyliber-command-center-1bym.onrender.com/api/health). Wait for a cold start if necessary. It must report `status: "ok"` and `version: "0.3.0"`. A waking page or older version is not acceptance.
3. Sign in and open **Command Center → Live State Monitor**. Confirm release **0.3.0** and record the running deployment commit. It must be the tested release merge, or a later reviewed main commit retaining it. The public landing page also shows **v0.3.0**; neither label alone proves the exact commit.
4. Open **Mathematical Playground** and confirm the four checks still pass: x=1/y=4/z=3 gives direct 2/detour 4; Reveal shows the complete general definition; the synthetic `.tex` prompt downloads; the archive package downloads and stable replay link opens. Use [metric-spaces-test.tex](../examples/metric-spaces-test.tex) for the prompt check.
5. Return the deployed commit, health version, Live State version and four-check outcome to Sol. If a check fails, give its step and visible error. Sol records the evidence or fixes the concrete failure; never infer a live result from GitHub merge alone.

The evidence above establishes maintainer-reported end-to-end application v0.3.0 acceptance. Sol subsequently published [v0.3.0 — Reviewed mathematical playground](https://github.com/GyLiber/gyliber-command-center/releases/tag/v0.3.0) at the exact accepted deployment commit, alongside retrospective v0.1.0 and v0.2.0 releases. All three remote lightweight tags were verified; no cryptographic tag signature is claimed. Repository publication and site deployment remain separate milestones, so this documentation update requires no new deployment or retest. Future releases must retain their checked source, live acceptance and verified tags; see [release tag rules and provenance](RELEASE_TAGS.md).

## Next work

Judge the pilot's usefulness before scaling it. Review one permitted actual course-source proposal with complete definitions and honest visual mapping. Bounded data-only recipes can follow practical evidence; general executable imports and proof support need their own designs/reviews. The temporary PostgreSQL expiry **2026-10-30**, durable backups, signed-in restart continuity and wider audit/contract prerequisites remain separate open work in [Next Steps](NEXT_DEVELOPMENT_STEPS.md).
