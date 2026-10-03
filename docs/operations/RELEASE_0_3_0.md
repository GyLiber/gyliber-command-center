# Application v0.3.0 — Reviewed Mathematical Playground

Prepared by **Sol**, continuing Luna's foundation, at Gyile's instruction on 2026-10-03.

**Status: release source prepared; final hosted version/revision acceptance pending.** This development is the next minor application release. It does not require completion of every item in the longer-term roadmap. A successful build alone is not an end-to-end release demonstration.

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
| Hosted pilot behavior | Gyile reported **“All four passed”** on 2026-10-03 for signed-in metric/definition/prompt/archive checks. Exact deployed revision not supplied; this is maintainer-reported evidence |
| Release identity change | Shared Cargo-derived page labels and package/lockfile version 0.3.0; PR/check/merge evidence to be recorded after verification |
| Hosted application 0.3.0 identity | **Pending:** deployment commit, health version and signed-in Live State comparison |
| GitHub version tag/release | Not yet published; publish only against the accepted release commit |

The prior cloud-browser URL-policy block prevents Sol from certifying this hosting step through that browser. No bypass or inspection of secrets is needed. The [manual-authoring runbook](MATH_PLAYGROUND_MANUAL_AUTHORING.md) retains the functional test and replay details.

## Final steps for Gyile after the release PR is merged

1. Open the existing Render web service for [the live site](https://gyliber-command-center-1bym.onrender.com). In **Deploys**, compare the deployed commit with the release PR's merge commit. If needed choose **Manual Deploy → Deploy latest commit**, then wait for **Live**. No new account, environment variable or paid API is required.
2. Open [health](https://gyliber-command-center-1bym.onrender.com/api/health). Wait for a cold start if necessary. It must report `status: "ok"` and `version: "0.3.0"`. A waking page or older version is not acceptance.
3. Sign in and open **Command Center → Live State Monitor**. Confirm release **0.3.0** and record the running deployment commit. It must be the tested release merge, or a later reviewed main commit retaining it. The public landing page also shows **v0.3.0**; neither label alone proves the exact commit.
4. Open **Mathematical Playground** and confirm the four checks still pass: x=1/y=4/z=3 gives direct 2/detour 4; Reveal shows the complete general definition; the synthetic `.tex` prompt downloads; the archive package downloads and stable replay link opens. Use [metric-spaces-test.tex](../examples/metric-spaces-test.tex) for the prompt check.
5. Return the deployed commit, health version, Live State version and four-check outcome to Sol. If a check fails, give its step and visible error. Sol records the evidence or fixes the concrete failure; never infer a live result from GitHub merge alone.

After this evidence is recorded, application v0.3.0 is demonstrated end to end. Optionally create the GitHub release from **Releases → Draft a new release**, tag `v0.3.0`, select the accepted deployment commit as target, title **v0.3.0 — Reviewed Mathematical Playground**, and use this document/changelog for notes. Do not select an untested newer main revision or describe pending proof/provider/operational capabilities as released. The connector currently has no tag/release-creation operation; repository publication and site deployment are separate milestones.

## Next work

Judge the pilot's usefulness before scaling it. Review one permitted actual course-source proposal with complete definitions and honest visual mapping. Bounded data-only recipes can follow practical evidence; general executable imports and proof support need their own designs/reviews. The temporary PostgreSQL expiry **2026-10-30**, durable backups, signed-in restart continuity and wider audit/contract prerequisites remain separate open work in [Next Steps](NEXT_DEVELOPMENT_STEPS.md).
