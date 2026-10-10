# Resolution Control operations

**Implementation checkpoint:** hosted deadline-only pilot deployed on 2026-10-09. Last accepted/tagged application version remains **0.3.0**; the live source contains unreleased Resolution Control candidate **0.4.0**. Owner import and acceptance remain pending.

## Current checkpoint — 2026-10-10: actual dates onboarding, pending member restore

The prior [All-Deadlines feature PR #51](https://github.com/GyLiber/gyliber-command-center/pull/51) merged at `f11cfd619e7844821d003794dde8f7702f7ba4f9`; Render [deploy `dep-db57grjbc2fs73ejlij0`](https://dashboard.render.com/web/srv-dau0307lot8c7395m6vg) reports **live** at `2026-10-10T17:42:57.604914Z`. Gyile personally confirmed the member **All deadlines** section appears but contains **no deadline records**. A page existing is not evidence that actual records are populated. No release tag change.

**Confirmed owner-approved minimal schedule:** A privately held local v1 JSON backup contains 1 resolution, 15 actual dated course commitments and 1 **internal 23 October preferred finish target** (16 commitments, 17 sequential create events, zero actions or threats). The SHA-256 digest matches the locally assembled Rust JSON tuple format; **server-side restore has not yet been demonstrated**. All dates originate in the user's supplied source document, not independently verified university notices. Its private document and the JSON file must not enter public GitHub. The unresolved CS344 sick test date remains omitted, not guessed.

**Display correction:** The initial overview categorized the internal target under `Date unknown`, because it stored that date in `schedule.earliest_finish` with a null hard deadline. The bounded fix branch `fix/resolution-target-only-overview-2026-10-10` uses a target-only date when no hard deadline exists, labels it **Internal/preferred finish target · no hard deadline**, counts it separately, and keeps chronological ordering. The record must never be presented as an official institution deadline. Existing owner isolation, data model, migrations, CSRF and authenticated restore remain unchanged.

**Next manual owner action (required for useful populated live pilot):** With the fix deployed only after exact-head CI/Security, open `https://gyliber-command-center-1bym.onrender.com/command/resolution-control` in the existing signed-in member session. First check that **workspace is truly empty: 0/16 outcomes, 0/64 obligations, revision 0**; *no data in All deadlines alone is not proof of empty storage*. If any preexisting records/history exist, stop; do not purge. In the private recovery section click **Download latest ledger only** and retain that authentic metadata file, then **Restore into an empty workspace**: select the conversation-held `resolution-control-DEADLINES-ONLY-pilot-v1.json` and the freshly downloaded separate ledger, check acknowledgment, click **Restore reviewed backup**, confirm. Never use a fabricated empty ledger or direct SQL insertion. If save becomes uncertain, retry **the identical request**, not a new one. On server success expect revision 17, 1 outcome, 16 obligations (15 deadline, 1 target-only), no invented actions/threats; check key official dates and 23 October internal target display. Download a fresh private backup **and** newest separate deletion ledger to independent storage. Report any error code verbatim, without secrets.

**Next engineering gate:** Observe Gyile's actual import, signed-in dashboard, keyboard/mobile comfort, correct SAST calendar/time and refreshed sessions; investigate any guard/replay/checksum error rather than bypass it. Formal v0.4.0 tagging/release awaits accepted end-to-end functionality plus intentional disposition of the expiring free PostgreSQL 18 database (expires `2026-10-30T08:00:20Z`). Until that deadline, keep this strictly disposable and independently backed up. Do not import high-value privacy records. After the documentation and deployment checkpoint, **stop for today**.

## Deadline overview — 2026-10-10 UI batch

The member view now includes **All deadlines** between Current Action and the obligation ledger. It reads the same owner-bound bootstrap workspace; **no new endpoint, database table, private import, action command or permissions** are added. Each recorded commitment appears exactly once, grouped as **Overdue, Today, Next 7 days, Later, Date unknown**. Dated groups sort ascending by South African local calendar day and then by recorded exact time; date-only records show **Time not specified**, sort before known times on the same day, and are never interpreted as midnight. Unknown dates are retained and never assigned invented deadlines.

Status is a *time-proximity indicator*, **not** an assertion of readiness, completed work or official university authority. Proximity is calculated using the backend's `view.observed_at` and a fixed UTC+02:00 South African display offset. The client clock, timezone and running timers do not silently change authoritative status. Use **Refresh state** to obtain a fresh observation. One-read snapshot and explicit observation label prevent unexplained stale state. The display contains no HTML interpolation of private data. **View obligation** focuses and scrolls to the existing detail card; it does not mutate state or select an action. Empty workspaces get an explicit empty message, and authenticated session loss immediately clears the overview.

Verify the pure categorization and ordering in `resolution-deadlines.test.mjs`, and run the synthetic Chromium fixture for 390 px, exact SAST midnight rollover, same-day past and date-only behavior, keyboard focus navigation, text-only rendering and session-loss erasure. These tests do not replace member-hosted verification or the pending deadline-only private import. Feature code is a **proposed v0.4.0 improvement**, not a new release/tag; no Render deployment occurs in this PR. Resume the existing live-pilot acceptance procedure in the 2026-10-09 checkpoint before claiming the new overview is deployed.

## Live disposable deadline-only pilot activation checkpoint — 2026-10-09

**Current runtime, superseding historical disabled/preview instructions below:** Render deployment `dep-db47a1dg1s2s738gd2d0` of `df0721db8dc29ddae7285deb88e0383bb4db5523` was marked `live` at `2026-10-09T05:01:39Z`; the actual feature environment flag is now `RESOLUTION_CONTROL_ENABLED=true`. Startup logs found/checked Resolution Control PostgreSQL schema and reported the web service live. Route: `https://gyliber-command-center-1bym.onrender.com/command/resolution-control`. This is **not** a verified authenticated import, secure backup, usability or release acceptance; the previous passages below describe earlier development stages and must not be treated as today's operational state.

**Minimal data agreement:** Only real dates, times, course/module labels and a clearly marked *internal* target are eligible for this **disposable** pilot: a generic resolution, 15 dated assessment/tutorial items, one internal readiness target (16 commitments total). No detailed threats, livelihood/academic narrative, invented next actions, imported verification evidence or arbitrary readiness assertions. The full source note explicitly bars committing it to public GitHub. Both original data and local import JSON remain outside source control; there is no owner-authenticated import acceptance yet.

**Next safe member procedure:** First sign in as the member at the route and verify whether the workspace is empty (revision 0, no records). If not empty, STOP and preserve the existing state. For an empty workspace only, inspect/download the latest independent deletion ledger; use the protected restore UI with the local deadline-only JSON backup and the newest ledger, review its warnings and confirm the operation. After server acknowledgement/reload verify 1 resolution, 16 commitments and **0** fabricated actions or threats; inspect every deadline and confirm the 23 October target is labelled as internal. Download a fresh private export and the latest independent deletion-ledger file into trusted storage outside the expiring service. Do not use the account owner's raw login cookies, direct SQL bypass, fake positive test reports or public GitHub fixtures with actual schedule details.

**Retention / release boundaries:** The existing Render free PostgreSQL 18 expires `2026-10-30T08:00:20Z`; no independent restore to a second environment has yet been demonstrated, and the Render hosted SQL integration cannot inspect it because its current external IP allowlist is empty. The external restriction remains unchanged. No higher-sensitivity data should be onboarded and no assumption of durability is justified. Application **v0.3.0** remains the last formally accepted/tagged release. New code is already hosted, but v0.4.0 remains proposed until actual member acceptance, backup/recovery and an intentional storage decision. **No work beyond this documentation checkpoint today.**

## Availability and identity

`RESOLUTION_CONTROL_ENABLED` defaults to `false`; only explicit `true` or `false` is accepted. Keep it disabled during this packet. Startup applies migration `0003_resolution_control.sql` when PostgreSQL is configured, including while disabled. No in-memory fallback is provided. Disabled or unavailable storage produces HTTP 503. No new service account or paid AI is needed.

The existing GitHub member allowlist/session remains the entry boundary. Resolution Control additionally derives the owner from the authenticated GitHub numeric user ID (`github-<id>`), preserving ownership across login-name changes. Old sessions missing that ID receive `reauthentication_required`; sign out and sign in again when the feature is activated. Requests cannot select an owner. Each owner has one independent bounded workspace; sharing/delegation is deferred.

## API contract for the member interface

All routes are under `/api/resolution-control`. Responses inherit the application security headers and `Cache-Control: no-store`. Reads require membership and same-origin browser context. Writes additionally require the session-bound `x-resolution-csrf` token from bootstrap. Browser cross-site/same-site subrequests are rejected; a supplied Origin must equal the configured OAuth callback's origin. No credentialed CORS is added.

| Method | Suffix | Outcome |
|---|---|---|
| GET | (empty) | CSRF token, current generation/revision, authoritative time, guarded state, buffers and limits |
| POST | `/commands` | Apply one typed domain command and append its private event atomically |
| GET | `/history?after_revision=0` | Up to 100 subsequent command events |
| GET | `/report?date=2026-10-08&offset_minutes=120&cutoff_revision=1` | Private daily snapshot and changes; cutoff is optional |
| GET | `/export` | Versioned JSON backup attachment, not executable code |
| GET | `/recovery-metadata` | Separate latest deletion-marker attachment |
| POST | `/restore` | Validate/replay a backup into an empty workspace under the current owner |
| POST | `/purge` | Delete private event payloads and rotate the workspace generation |

A first command body, using synthetic data only:

```json
{
  "meta": {"operation_id": "unique-request-1", "generation": null, "expected_revision": 0},
  "command": {
    "type": "create_resolution", "id": "resolution-1",
    "spec": {"title": "Synthetic outcome", "objective": null, "client_reference": null}
  }
}
```

For later writes use the generation/revision returned by bootstrap. Assign a fresh operation ID for a new deliberate action. Retry an uncertain request with its **identical body and operation ID**. Identical retries return the stored acknowledgement without appending a second event. Reusing an ID for a different body or a stale revision returns 409; reload and review rather than silently replacing data. After every acknowledgement, refresh the current view: a replayed acknowledgement can precede later successful edits. A generation rotates on restore/purge, invalidating pending edits and old-generation retry results. Previously acknowledged operations against a replaced generation return 410. Unknown/foreign record references return the same 404. Invalid transitions return 400 without echoing private input.

The command variants in `src/resolution_control/workspace.rs` are the canonical typed schema. They cover resolutions, commitments/schedules, action plans/focus/lifecycle, scope, mapping/deployment, stress tests/verification, readiness confirmation/reopening and threats. Only the selected action can start, and only one action can run per owner. Completion records an existing artifact reference and clears that focus; no next action is selected automatically. Action completion is separate from deliberate readiness confirmation. Evidence identity, revision and live timestamp come from the server.

## Persistence and limits

Migration 0003 creates owner-filtered workspace headers, append-only command events, idempotency acknowledgements, source-generation roots, deletion barriers and minimal audit buckets. An owner transaction lock serializes reads and writes. State is deterministically reconstructed from the validated command history; header revision/byte count and appended event commit together. PostgreSQL time supplies whole-second UTC timestamps, clamped to the previous event time during small backwards clock corrections. The application never accepts a live mutation timestamp from a browser.

Limits per workspace: 16 resolutions, 64 commitments, 128 actions, 128 threats, 128 unique scope/dimension pairs per commitment, 2,048 events and 3 MiB of serialized command payloads. Regular request bodies are limited to 64 KiB; restore bodies to 4 MiB, including backup/metadata wrappers. Scalar limits and safe-reference rules remain in design section 18. Capacity failures preserve state; export before deliberately resetting a workspace. No silent event pruning or overwrite is provided.

History stores control metadata and references, not original coursework or artifact bytes. It is private application data: never commit its export or a real pilot instance to this public repository. References are not fetched automatically. Render/request tracing must not log bodies, report contents or exported data. Export files are unencrypted JSON; retain them privately with appropriate encrypted storage/access controls.

Ordinary successful commands retain the event code (derived from the command), owner-derived actor context, revision, changed values and database timestamp. Administrative restore/purge and access/operation denials retain minimal stable codes, actor and one-minute occurrence buckets. A denial cannot be durably audited without working PostgreSQL; with a configured failing pool it returns unavailable. This bounded module does not complete the wider company authentication/contract audit roadmap.

## Reporting precision and provenance

The daily report derives a snapshot and changes from one consistent event history. The optional cutoff revision freezes the chosen prefix; the local-day interval includes its start and excludes the next midnight. Completed outputs, evidence, control changes and unresolved work/threats are available from that same snapshot. An empty day explicitly has no recorded changes. The API emits data for a private preview/download; it never sends or publishes a client report.

The first implementation accepts an **explicit fixed UTC offset** in minutes, default +120 (South Africa), range -840 through +840. It does not implement named timezone/DST rules. The interface must display that offset and never describe it as automatic timezone inference. Report state is bounded by the end of the requested day and observed database time. Administrative restore/purge audit buckets are separate from the command-derived report. Imported entries retain their original claimed times and receive `origin=imported`; the report and bootstrap identify imported history. Such records are user-supplied claims, not independent historical actor/time authentication or proof certification.

## Recovery, deletion and retention

Active event history is retained until the owner deliberately purges it, subject to the finite capacity above. Idempotency records and minimal audit buckets have a 30-day policy, cleaned opportunistically during operations/audit writes; this is not a scheduled purge service. Old backups remain private copies under the owner's control. Delete superseded local/offline copies deliberately; an application purge cannot erase a downloaded file or a host's database backups.

Purge requires current metadata and the exact confirmation `DELETE MY RESOLUTION CONTROL DATA`. It removes all private command-event payloads, rotates generation and leaves minimal deletion barriers and operation/audit metadata. The acknowledgement includes a newest recovery-metadata receipt. Retain deletion barriers indefinitely while corresponding backups may exist; the current API deliberately has no marker-retirement action. Metadata is bounded to 4,096 deletion generations and backups to 128 source roots. Repeated restores preserve roots without growing one root for each concurrency-token rotation.

Restore requires `meta`, `backup`, `recovery_metadata` and explicit `confirm_recovery_metadata=true`. It never overwrites nonempty work. Export schema 1 includes commands, revisions, times, source generation, lineage and checksum. Restore validates size, fields, schema, integrity, contiguous revisions, chronology and every domain transition, then assigns the current authenticated owner and labels all entries imported. It rotates the concurrency generation. It cannot inject executable code, owner, evidence actor or an arbitrary readiness status. SHA-256 detects accidental changes; it is **not** a signature or protection against an owner intentionally modifying a file.

To recover safely:

1. Retain the workflow backup and the **separate newest recovery-metadata file**, privately and independently of Render. Refresh the metadata after every deletion and retain the purge receipt. Do not assume an older backup knows about a later deletion.
2. In a disposable test environment with an empty migrated database, sign in again. Workflow exports do not restore sessions or secrets. Obtain bootstrap metadata/token.
3. Provide the selected backup and the latest independent recovery metadata, review imported provenance and confirm the metadata is current. The API rejects source roots listed as deleted. Never bypass that check by providing an older deletion ledger.
4. Compare reconstructed scope, actions, evidence/history and a known report cutoff with the source export. Test a previously deleted backup is rejected. Retain private test evidence and actual recovery time.
5. Only then decide whether to onboard a disposable pilot or arrange durable storage. Recovery completion does not automatically authorize higher-classification data.

A fresh database cannot discover deletions that happened after an old backup unless the latest independent ledger is supplied. The application relies on that explicit recovery procedure; it does not offer tamperproof or automatic backup guarantees. Never restore the old database wholesale and assume newer deletion decisions survived. Deletion receipts do not authorize publication of private workflow records.

## Verification and next activation gate

The packet adds synthetic aggregate, replay, report, HTTP and PostgreSQL integration tests. GitHub CI supplies PostgreSQL 18 and exercises actual persistence, reconnection, owner isolation, duplicate retries, concurrent conflicts, atomic rejection, restore/purge barriers, HTTP limits and failed-pool behavior, persisted CSRF-denial audit and the private HTTP report. Without `DATABASE_URL`, database tests skip their bodies; that local result alone is not database verification. Required exact-head CI/Security evidence belongs in the implementation PR/checkpoint.

The next packet implements the calm dark member page/registry and browser workflow, including refresh/conflict/error states, imported labels, visible limits and deliberate backup/delete/restore controls. No manual Render change is requested in this API packet. Before activation, follow the hosted acceptance packet: resolve the temporary free PostgreSQL expiry **2026-10-30** through durable hosting or an explicitly disposable synthetic pilot, prove independent restore/session behavior, verify deployed identity and ask Gyile to demonstrate the workflow/usefulness/eye comfort. Keep `RESOLUTION_CONTROL_ENABLED=false` until those prerequisites are ready. Application v0.4.0 is released only after its scoped end-to-end acceptance; no release tag changes here.


## Member preview packet — 2026-10-08

The smaller first UI packet adds `/command/resolution-control` and a Confidential preview registry entry. Anonymous page requests redirect to login; protected API calls still enforce stable owner identity, same-origin context and CSRF. A member may open the preview page while disabled, but its fields begin disabled and the API explains unavailability. The registry link stays reserved unless feature/storage/identity are available. Keep the flag disabled; no deployment action is requested in this packet.

Synthetic usage in a prepared test environment:

1. Capture a resolution, one commitment and one action. Unknown controls can stay blank. Select deadline/target precision explicitly; exact date/time inputs and observations display UTC+02:00 with whole seconds. Date-only inputs remain date-only.
2. Select the action from the ledger. Add an expected output and verification method using Edit plan; mark ready, then start. Record a reference to its completed output. Completion clears the current focus without selecting another or claiming readiness.
3. Refresh to observe authoritative state/buffers. Editing controls captures a revision; refresh does not silently rebase that draft. After a conflict, review the current item and deliberately reload it before editing/saving again. Existing typed draft values remain visible.
4. If a save is uncertain, keep the tab open and use Retry the same request. It keeps the identical command/operation metadata, so an already accepted save is acknowledged without duplication. No automatic retry occurs. A failed refresh blocks further writes; a confirmed save with failed refresh is labelled accepted.

All original artifacts remain in their existing tools. Web references forbid credentials/query/fragment tokens; other references are non-clickable text. Captured strings render as text, not HTML. Private drafts and uncertain-request recovery exist only in tab memory; no local/session storage or trace logging is used. Known session loss/account changes clear the display/drafts. Reloading the tab discards unsaved drafts and its retry envelope, so resolve an uncertain save first.

The UI deliberately labels itself a synthetic preview: scope/evidence/threat editing, private report/history and deliberate export/restore/purge controls follow in the **next** packet. The underlying API remains implemented and tested. Browser tests use a controlled synthetic API fixture; Rust/PostgreSQL tests cover the real service separately. Desktop/mobile CI screenshots support visual review, but Gyile's eye comfort and live workflow usefulness remain later acceptance checks.


## Member Packet B — private reports and recovery controls, 2026-10-09

Implementation branch: `feat/resolution-private-report-recovery`. This adds the member interface for already-existing owner-bound API routes. No new persistence, provider, session type, or public data store is introduced.

- **History:** on-demand bounded 100-event pages using `after_revision`; strictly increasing revisions; stale lists cleared when current workspace revision/generation changes. Imported events are marked user-supplied, never proof-certified.
- **Daily report:** explicit local date, fixed UTC offset in whole minutes [-840, 840] (South Africa +120), and optional history revision cutoff. Server derives all data from one consistent event snapshot. Preview shows event codes, recorded human outcomes, completed output references and unresolved threats. Empty days are explicit. Download is a **separate deliberate private JSON action**; no automatic message or publication.
- **Backup:** request versioned owner-bound export, then the independent deletion-ledger metadata and a final current-state read to detect locally observable concurrent changes. Browser downloads both as separate unencrypted JSON files. This cannot prevent another tab changing state after the final check; always retain the **newest** metadata after a purge. Never put private files in public Git history.
- **Restore:** select backup and the newest independently retained deletion ledger, review the explicit acknowledgment, require an empty destination, send a bounded CSRF/idempotent `POST /restore`. Backend validates checksum, replay, timestamps, lineage/deletion markers and owner context. Imported actor/timestamp fields are historical claims only. No in-place overwrite or implicit identity/session recovery.
- **Purge:** exact phrase `DELETE MY RESOLUTION CONTROL DATA`, additional browser confirmation, protected `POST /purge` and automatic private download of the acknowledgement's latest deletion receipt (with an explicit second-download option). Acknowledged deletion rotates the generation; uncertain acknowledgements retain the exact request ID/body for explicit retry. Purge cannot delete previously exported files or host backups.
- **Privacy and failures:** user-provided strings remain text nodes; no HTML interpretation, localStorage, hidden background retries or client-side authorization. Session loss clears displayed history/reports/upload inputs/receipts. Conflict/replaced-generation responses require refresh/review. No action is treated as saved until the server returns acknowledgement; a failed post-ack refresh remains visible.

Review with Node bounds/schema/cursor tests, synthetic desktop/mobile Chromium scenarios, and existing Rust PostgreSQL/API/container/security gates. Controlled browser fixtures are not hosted integration proof. Exact-head CI/Security evidence and any limitations belong in the implementation PR before merge. Release **v0.3.0** remains accepted; proposed **v0.4.0** cannot ship until Packet C hosted acceptance. Keep `RESOLUTION_CONTROL_ENABLED=false`, no real private onboarding. Temporary database expiry **2026-10-30** remains an explicit storage/restore gate.

Next after Packet B merge/stop: **Packet C hosted synthetic acceptance and release** — actual deployed revision and member identity/session verification, private clean-database restoration with latest deletion ledger, adverse deletion/restore boundary, durability vs disposable pilot decision, Gyile's utility/eye-comfort checks and only then an application v0.4.0/tag decision. Request manual Render actions at that point only.
