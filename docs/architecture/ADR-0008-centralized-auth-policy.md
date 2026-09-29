# ADR-0008: Centralized Authenticated-Member Policy

**Status:** Accepted  
**Date:** 2026-09-29  
**Release:** v0.2.0 development

## Context

Protected Command Center routes were independently checking the session for an authenticated member. Repeated checks are easy to drift when new modules are added.

The Command Center also has two distinct protected surfaces:
- browser pages, where an anonymous visitor should be directed to the login boundary;
- APIs, where an anonymous caller should receive an explicit authentication failure.

## Decision

Centralize authenticated-member policy in `src/auth.rs`.

The policy exposes:
- `require_page_member`
- `require_api_member`
- `login_redirect`
- `authentication_required`

Handlers therefore consume one authentication policy contract instead of duplicating response semantics.

The underlying membership rule remains the existing explicit GitHub allowlist. No role escalation or authorization widening is introduced.

## Security rules

Authentication remains separate from authorization.

The current v0.2 policy is:
`authenticated member -> permitted to current low-risk internal surface`

As sensitive domains are activated, the policy must evolve to:
`actor + resource + action + classification + policy`

No sensitive operation may rely on hidden UI controls.

## Consequences

### Positive
- One place defines anonymous page/API behavior.
- New protected modules have a standard access contract.
- Authorization policy can evolve independently from route rendering.
- Tests can lock the failure semantics.

### Future

Role-based or attribute-based authorization may be introduced before restricted finance, personnel, secrets or critical IP modules are activated.

Step-up authentication should be required for high-impact operations in the mature platform.
