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
