# Professional Knowledge Gap Register

This is a living engineering memory document for knowledge that is easy to miss when building a company command center.

## Security

Rust reduces memory-safety risk; it does not make a web system unbreachable. Security depends on identity, authorization, dependencies, cloud configuration, secrets, data design, operations, people and recovery.

Authentication proves identity. Authorization decides what that identity may do.

The browser is untrusted. Every protected operation must be authorized server-side.

## Data

A database is not a backup. Independent copies, retention, encryption, monitoring and successful restore tests are required for critical information.

Credentials are not ordinary application data. Bank keys, GitHub tokens, cloud credentials, signing keys and OAuth refresh tokens require dedicated secret-management controls.

Every important live value eventually needs provenance and freshness semantics.

## Reliability

A free hosting plan is useful for development but is not automatically a production archive. Provider failure, account loss, accidental deletion, region failure and ransomware must be considered.

Define RPO and RTO per data class. Test recovery rather than assuming backups work.

## Supply chain

Dependencies are part of the attack surface. Lock dependencies, monitor advisories, review material updates, scan release artifacts and maintain provenance/SBOM evidence for serious releases.

## Operations

Observability should expose service health, latency, failures, authentication events, authorization denials, integration freshness, backup state and deployment health.

Audit records are different from debug logs: audit records answer who did what, to which resource, when, under what authorization context, and with what outcome.

## Privacy and legal/IP

Public source visibility does not grant reuse rights to proprietary GyLiber material.

Future personnel, customer, contract, copyright, licensing, financial and strategy records require explicit classification, retention and access policies.

## AI-assisted development

AI output is proposed engineering work. It receives ordinary review, testing, security, dependency and IP/licensing checks. AI assistance never becomes a bypass around human release accountability.

## Architecture

Avoid both extremes:
- one giant unbounded application
- premature microservices before a real boundary exists

Extract a module when security, performance, deployment ownership, fault isolation or team-scale coordination creates a measurable reason.
