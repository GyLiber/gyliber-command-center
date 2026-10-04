# Application release tag history

Prepared by **Sol**, continuing Luna's foundation, on 2026-10-04 at Gyile's explicit request to backfill the missing minor-release tags.

**Publication status: pending.** No remote tags or GitHub releases existed at the audit. The current connector cannot create tags, and the shell's Git connection is unauthenticated. This document records reviewed targets; it does not claim publication. Tagging changes no application code and requires no Render deployment.

## Reviewed targets

| Application tag | Exact commit | Reason for this release boundary |
|---|---|---|
| `v0.1.0` | `c8546dbb786f1a865de2ba12086f6dcd89126655` | PR #4's verified original foundation, before subsequent module development. A retrospective source-release milestone; independent historical hosted acceptance is not established. |
| `v0.2.0` | `b134635a7b781e9e3f05721e54be8959f6a6fab8` | PR #19's final documented live-product/canonical-identity checkpoint, including the CA-certificate deployment correction and package version 0.2.0. The subsequent contracts/session/playground development belongs to the next line. |
| `v0.3.0` | `ae80a6d04b0753dd4f6bd6a90f75dc003ec37d3d` | PR #34's final release-evidence commit: the exact deployment Gyile confirmed Live with passing playground tests, health/Live State version 0.3.0 and health status ok. PR #35 later records that acceptance. |

Each target is on main's ancestry, its Cargo version matches its proposed application tag, and its complete tree equals the corresponding successfully checked PR head. Targets are ordered ancestors of one another. Do not use today's main for all three tags, infer additional minor releases from feature commits, or confuse exhibit/capability versions with application releases.

## Verification retained

- v0.1.0: checked PR #4 head `e219c06d63fb82201d61e5ffc8772601c1ea2215`; [CI 36576368703](https://github.com/GyLiber/gyliber-command-center/actions/runs/36576368703) succeeded. Security automation was introduced later; no historical Security workflow is claimed here.
- v0.2.0: checked PR #19 head `6429d73fd209c89f91934011759a98a2e65353cd`; [CI 36618755924](https://github.com/GyLiber/gyliber-command-center/actions/runs/36618755924) and [Security 36618755970](https://github.com/GyLiber/gyliber-command-center/actions/runs/36618755970) succeeded.
- v0.3.0: checked PR #34 head `ffd283b8a04bfd801c691e03a8e821dd4f1b084a`; [CI 37113789117](https://github.com/GyLiber/gyliber-command-center/actions/runs/37113789117) and [Security 37113789121](https://github.com/GyLiber/gyliber-command-center/actions/runs/37113789121) succeeded. See [live acceptance](RELEASE_0_3_0.md).

## Publication rules

Create each tag only at the exact reviewed commit. Prefer annotated Git tags when authenticated Git/API tooling is available; otherwise disclose the tag type actually created and retain release notes/provenance. Retrospective creation dates must remain truthful: do not backdate tag creation or claim cryptographic signing without a real signature.

After publication, fetch the remote tag refs, peel any annotated objects, and verify each commit against the table. If a tag already exists at a different commit, stop and report the conflict; never force-move or delete published release tags. Add the verified tag links/type to this document and update the current release record's publication status.

For subsequent application releases, retain the checked source commit, live acceptance and release notes, then publish and verify its tag as part of closing the release. Tag creation and an optional GitHub Release listing are separate from Render deployment.
