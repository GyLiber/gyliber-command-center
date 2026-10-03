# Development continuity: Luna to Sol

**Date:** 2026-10-01
**Client/maintainer:** Gyile / GyLiber
**Continuing AI engineering collaborator:** Sol
**Inherited checkpoint:** PR #22, `fix/include-migrations-in-docker`

## Attribution and authority

At Gyile's explicit request, Sol is continuing development from where Luna left off and moving the Command Center toward its next required actions. Luna and Sol are the conversational identities used by Gyile for the AI engineering collaborators; GitHub commit authorship remains the authenticated contributor identity. This record does not rewrite or independently certify the authorship of earlier commits.

The continuation uses the repository's current requirements, source, roadmap and verification evidence. AI assistance remains subject to ordinary review, testing, security, dependency and release requirements. Human accountability remains with Gyile / GyLiber.

## State inherited

- `main` at `d6d52c12ac221e2f233b5e9f30e4c20d5856a20d` contains the durable PostgreSQL session implementation merged through PR #21.
- PR #22 at `cea402171b9444a14bdf20248111b4d7580084d7` supplies the missing Docker migration directory and documents the failed hosted build.
- CI run `36691043111` and Security run `36691043168` passed for that PR head. Those runs did not build or start the production Docker image.
- The inherited documentation reports that Render's private `DATABASE_URL` was configured, while durable-session hosted activation remained unverified. Sol has not independently inspected that secret or configuration.

## First Sol work unit

Complete the existing deployment-correction checkpoint before expanding the application domains:

1. Preserve Luna's migration-copy correction.
2. Build with the committed `Cargo.lock`, `--locked` and Rust 1.98.1 to match CI.
3. Add a production-container CI gate using an empty, disposable PostgreSQL 18 database.
4. Verify production refusal without a database, automatic migration, HTTP health, packaged static assets, anonymous access controls, OAuth session persistence and a second startup against the existing schema.
5. Restore the older changelog history accidentally removed in PR #22.
6. Update verification and roadmap documentation, then use the existing PR workflow for review and merge after exact-head checks pass.

The test starts OAuth with synthetic configuration to exercise session writes; it does not follow GitHub's redirect. A database row surviving restart is evidence of persistent session metadata, not a complete allowlisted-user login/logout or authenticated-session continuity demonstration.

## Remaining acceptance and next work

The new CI gate establishes build/startup evidence in a disposable environment. It does not prove Render deployment, live OAuth configuration, managed-database durability, backup restoration or suitability for critical records. Hosted activation requires deployment identity, expected source revision and live acceptance checks in `docs/operations/DEPLOYMENT.md`.

After that checkpoint, the next planned domain is section 3.3 of `docs/operations/NEXT_DEVELOPMENT_STEPS.md`: durable, minimal audit/event metadata with stable event codes, authoritative timestamps and documented retention. Contracts and engagements remain gated.

The temporary PostgreSQL environment and deferred canonical-domain purchase retain their documented constraints. No production credentials, customer data or protected company records are introduced by this work.

## Playground continuation

PR #22 and design PR #23 passed their exact-head CI/Security checks and were merged on 2026-10-01. At Gyile's subsequent explicit instruction, Sol prioritized the member Mathematical Playground as the next product capability. ADR-0010 and the playground activation runbook document the implementation, its constrained AI authoring boundary and the additional credentials/live acceptance needed. This extends Luna's existing authentication, PostgreSQL and module foundations. It does not assert that the broader company audit/contract prerequisites have been completed.

On 2026-10-02 Gyile authorized resumption, supplied the live Render URL, reported working member login and saved secrets, and clarified that paid API billing was unaffordable. Sol verified the public health endpoint, corrected the browser fixture's range-input value and removed exception exposure from that fixture. Sol added explicit Gemini free-tier configuration with provider-bound consent and quota handling. Credential presence remains reported by Gyile rather than independently inspected; a real provider draft and live Git publishing remain additional acceptance gates.


## Scoped provider correction and stopping checkpoint, 2026-10-02

PR #24 is merged and its playground frontend was observed on the live Render service; public health and anonymous access boundaries passed. Gyile subsequently reported a real Gemini generation failure. At Gyile's direction, Sol limited this work unit to correcting the provider integration, updating documentation and recording the next visual minor release.

The correction uses established Gemini JSON-output fields and recognizes Google's structured invalid-key HTTP-400 response. Google's live schema also recognizes the previous format, so payload compatibility alone is not a proven root-cause fix; request logging remains disabled with store: false. Request/model/access/quota/timeout failures now have separate safe messages; logs contain fixed codes/status rather than provider prose or source material. The old generic failure cannot establish the production root cause because the upstream response was discarded. Credential configuration is reported, not independently certified; successful live drafting and Git publication are still acceptance gates.

After exact-head CI/Security and merge, deploy the latest `main` and perform one small synthetic Gemini draft test using the runbook. Record the actual result before calling hosted authoring complete. Do not enable paid billing or publish synthetic coursework as Gyile's work. Stop after this correction/documentation checkpoint. For the next session, implement [calmer dark visuals](../architecture/MATH_PLAYGROUND_DESIGN.md#next-minor-release-calmer-dark-visuals), preserving formal mathematics, reduced motion and immutable artifact versions. Theorem proofs remain deferred to capability 0.3.0; the wider audit/contract roadmap is unchanged.


### Retest narrows model access, 2026-10-03 (South Africa)

Gyile's retest after deploying PR #25 reported the specific model-unavailable error. Sol checked Google's current model-access notices and corrected the earlier new-project advice: Gemini 2.5 is restricted to previous active users; use explicitly configured `gemini-3.5-flash-lite` for this free-tier pilot. A small compatibility patch removes `candidateCount`, unsupported by Gemini 3, while preserving the single-candidate response check. See the [follow-up setup steps](../operations/MATH_PLAYGROUND.md#follow-up-model-access-failure-2026-10-03-south-africa). After exact-head CI/Security and merge, Gyile must update the private Render model/provider settings and rebuild/deploy latest main, then make one synthetic draft test. That work unit ended at the agreed stopping checkpoint. The later resumption below supersedes its temporary pause, without claiming live acceptance.

## Sol resumption and dark visual work, 2026-10-03

Gyile explicitly requested continued development directly in GitHub and pointed Sol to the Next Steps document. PR #25 and PR #26 had merged after exact-head CI/Security. Asked about the live model-change retest, Gyile replied **“Not tested yet.”** Real account authoring and runtime Git publication therefore remain unverified.

Sol continues Luna's foundation with the requested calmer presentation, independently testable through synthetic demonstrations. Exhibit bundle 0.2.0 uses a new dark renderer and pauses motion until Animate is chosen. Formal reveal remains 0.2.0; engine mathematics is byte-for-byte unchanged. The application release remains 0.2.0, with the v0.3 line unreleased.

Earlier 0.1.0 demo files and renderer are retained unchanged. Private older exhibits keep their stored bytes and initially show a dark palette-choice screen; an explicit dim view applies a display filter, while downloads/publication retain original code. New packages record the new renderer digest. No source migration or paid dependency is introduced.

The merge gate is the existing CI/Security workflow, including PostgreSQL old/new-byte preservation, versioned authenticated runtime routes, mathematical invariant checks and browser fixtures. CI supplies synthetic desktop/mobile screenshots for visual review. Hosted revision, personal comfort, successful Gemini authoring and live artifact publication are subsequent acceptance gates in the runbook. The wider durable-audit/contract roadmap remains unchanged.
