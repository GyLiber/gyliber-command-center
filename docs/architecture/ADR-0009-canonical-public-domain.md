# ADR-0009 — Canonical Public Domain and Hosting Abstraction

- **Status:** Accepted
- **Date:** 2026-09-29
- **Scope:** Public identity, DNS, deployment presentation and authentication callback
- **Decision owner:** Gyile / GyLiber

## Context

The Command Center is a business-facing GyLiber product whose current managed-host deployment provides a generated infrastructure hostname.

A provider-generated hostname is unsuitable as the permanent public identity because it is long, difficult to remember and tied to an implementation provider. It also creates unnecessary coupling between printed material and infrastructure.

The Command Center must be able to change hosting providers without changing its business-card address.

## Decision

GyLiber will use a **GyLiber-owned canonical HTTPS domain** as the public product origin.

Preferred presentation:

`https://gyliber.com/`

Fallback when the root domain is intentionally used by another corporate surface:

`https://hq.gyliber.com/`

The exact choice remains an external ownership/availability decision and is not fabricated by the repository.

The managed hosting provider remains infrastructure. Its generated hostname, service identifier, deployment identifier and dashboard remain engineering/operations references only.

## Consequences

### Positive

- Short, memorable address suitable for business cards.
- Public identity remains under GyLiber control.
- Hosting can change without changing printed material.
- OAuth callbacks can be defined against a stable corporate origin.
- Client-facing communication becomes independent of the infrastructure vendor.

### Required operational work

When the chosen domain is available and controlled by GyLiber:

1. Attach the domain to the current managed-host service.
2. Configure DNS at the authoritative DNS provider.
3. Verify the domain.
4. Confirm managed TLS.
5. Update the GitHub OAuth callback URL to the canonical origin.
6. Update `GITHUB_REDIRECT_URL` in the service environment to the identical callback URL.
7. Verify public pages, health and member authentication.
8. Use only the canonical domain in printed/client-facing materials.

## Security considerations

DNS and domain-account access become production security controls. Domain registrar credentials must use strong authentication and must not be stored in the repository.

The OAuth callback URL must be exact. Broad wildcard callback configuration is not part of this design.

The canonical domain does not change the authorization boundary: protected Command Center routes remain deny-by-default and require successful member authentication plus the GyLiber allowlist.

## Rejected alternatives

### Keep the generated hosting hostname

Rejected as the permanent public identity because it is infrastructure-coupled and poor for business-card use.

### URL shortener as the primary identity

Rejected as a foundational architecture because it adds a second service dependency and does not establish durable ownership of the public namespace.

### Provider-specific branded hostname

Rejected because the public identity would remain coupled to the hosting vendor.

## Future change condition

This decision may be revised when GyLiber establishes a broader corporate DNS architecture, multiple public applications, or a dedicated edge/proxy layer. The revised design must preserve stable public identity and provider independence.
