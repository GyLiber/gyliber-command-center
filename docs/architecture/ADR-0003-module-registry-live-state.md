# ADR-0003: Module Registry and Live-State Contract

**Status:** Accepted  
**Date:** 2026-09-29  
**Release:** v0.1.x

## Context

The Command Center is expected to grow into a multi-domain operational hub. Hard-coding every future capability into the main dashboard would make the UI and backend increasingly coupled.

The system also needs a consistent way to represent information that changes over time without confusing current observation with durable company data.

## Decision

Introduce two explicit contracts:

1. **Module registry**
   - A typed catalog declares module identity, display name, UI route, status, data classification and live capability.
   - The authenticated `/api/modules` endpoint is the runtime registry contract consumed by the dashboard.
   - Reserved modules may be declared before their data stores or high-impact capabilities exist.
   - A reserved module has no interactive UI route until that route is deliberately implemented.

2. **Live state snapshot**
   - The authenticated `/api/state` endpoint returns a typed low-risk operational snapshot.
   - The snapshot includes a schema version and server observation timestamp.
   - It contains no banking, personnel, production-secret, trade-secret or irreplaceable archive content.
   - Future live modules should follow the same principle of explicit freshness/provenance metadata.

## Consequences

### Positive
- New modules can be registered without rewriting the dashboard.
- The UI consumes declared capabilities rather than duplicating backend knowledge.
- Future domains have a clear place for access classification.
- Live information gains an explicit contract for freshness.

### Constraints
- The registry is not an authorization system by itself.
- Every protected API/module must independently authorize the current actor.
- A registry entry does not authorize persistence of sensitive data.
- The in-process catalog is intentionally static in v0.1.x.

## Future evolution

The registry may move into durable configuration or a database when module lifecycle management becomes a real operational requirement.

Live state may evolve from polling to SSE when the freshness requirements justify server-pushed updates.

Critical or restricted modules require stronger identity, policy enforcement, audit, storage and recovery controls before activation.
