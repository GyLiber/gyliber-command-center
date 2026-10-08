# Resolution Control operations

**Implementation checkpoint:** persistence/API packet, 2026-10-08, Sol. Application version remains **0.3.0**. This is preparation for the proposed **0.4.0** member workflow, not hosted acceptance or private activation.

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

The packet adds synthetic aggregate, replay, report, HTTP and PostgreSQL integration tests. GitHub CI supplies PostgreSQL 18 and exercises actual persistence, reconnection, owner isolation, duplicate retries, concurrent conflicts, atomic rejection, restore/purge barriers, HTTP limits and failed-pool behavior. Without `DATABASE_URL`, database tests skip their bodies; that local result alone is not database verification. Required exact-head CI/Security evidence belongs in the implementation PR/checkpoint.

The next packet implements the calm dark member page/registry and browser workflow, including refresh/conflict/error states, imported labels, visible limits and deliberate backup/delete/restore controls. No manual Render change is requested in this API packet. Before activation, follow the hosted acceptance packet: resolve the temporary free PostgreSQL expiry **2026-10-30** through durable hosting or an explicitly disposable synthetic pilot, prove independent restore/session behavior, verify deployed identity and ask Gyile to demonstrate the workflow/usefulness/eye comfort. Keep `RESOLUTION_CONTROL_ENABLED=false` until those prerequisites are ready. Application v0.4.0 is released only after its scoped end-to-end acceptance; no release tag changes here.
