# ADR-0005: Digital Resource Registry

**Status:** Accepted  
**Date:** 2026-09-29  
**Release:** v0.2.0 development

## Context

GyLiber will accumulate many digital assets: repositories, public profiles, service accounts, hosted applications, documents, internal systems and later classified resources.

Hard-coding links into individual pages does not scale. A durable command center needs a common identity and classification layer for assets before it needs a database.

## Decision

Introduce a typed Resource Registry.

Each resource declares:
- stable identifier
- display name
- resource kind
- lifecycle status
- visibility
- optional destination URL

The first implementation is static and public-data-only. The protected `/api/resources` endpoint provides the authenticated registry contract and `/command/resources` renders the discovery surface.

## Security

The registry is not an authorization mechanism.

A resource may become Confidential, Restricted or Critical later. Its classification must be enforced by the domain service and authorization policy that owns the resource.

External URLs are currently hard-coded by developers. Future database-backed URLs must be validated as allowed schemes/destinations before being rendered or requested.

Credentials are never stored in the registry.

## Extensibility

Future resource adapters may add:
- live status
- freshness/provenance
- provider-specific identifiers
- credential references held in a secret manager
- ownership
- audit metadata
- synchronization state

The registry should remain a metadata/control-plane contract, not a dumping ground for provider data.