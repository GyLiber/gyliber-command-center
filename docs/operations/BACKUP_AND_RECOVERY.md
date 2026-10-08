# Backup and Recovery Baseline

## Objective

A laptop failure, hosting failure, accidental deletion, compromised account or storage-provider outage must not destroy GyLiber's irreplaceable information.

## Recovery model

The eventual production design must separate:

1. primary application storage
2. independent backup storage
3. source/configuration recovery
4. secret/key recovery
5. documented restoration procedures

## Minimum controls for critical data

- encrypted backups
- versioning/retention
- credentials separated from application credentials
- monitoring for failed backups
- independent provider or failure domain where justified
- documented RPO
- documented RTO
- scheduled restore exercises
- evidence that the last successful restore is usable

## v0.1

No critical company archive is stored. Session state is transient and is not treated as a durable backup problem.

## Before Critical data onboarding

Create and approve:
- backup architecture
- encryption/key ownership model
- retention policy
- restore runbook
- restore test evidence
- account-recovery plan
- failure-domain strategy

A backup that has never been restored is an assumption, not verified recovery capability.


## Resolution Control development boundary

The proposed v0.4.0 module adds a private data-only workflow export plus a separate newest deletion ledger; see [its exact recovery and retention procedure](RESOLUTION_CONTROL.md#recovery-deletion-and-retention). A workflow backup alone cannot preserve deletions made after that backup. Restore into an empty migrated workspace requires current authenticated ownership, validated command replay and an explicit review of the latest independently retained ledger. Imported records are labelled user-supplied claims. Sessions and secrets are not in these exports.

The backend packet does not establish hosted backups, automatic encryption, RPO/RTO, private activation or permanent storage. Keep synthetic operation until the member workflow and hosted acceptance gates are complete. Resolve the temporary Render PostgreSQL expiry on 2026-10-30 before depending on it for a private pilot.
