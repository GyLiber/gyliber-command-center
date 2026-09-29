# ADR-0004: Public GitHub Repository Observability

**Status:** Accepted  
**Date:** 2026-09-29  
**Release:** v0.1.x

## Context

The Command Center is intended to represent digital assets in live or near-live form. The GitHub repository is itself a useful first asset because its public metadata can be observed without introducing company credentials or private data.

## Decision

The Command Center provides a protected read-only Repository Monitor module for the GyLiber Command Center repository.

The monitor:
- calls a fixed GitHub API endpoint server-side
- uses a five-second client timeout
- disables HTTP redirects on the shared HTTP client
- treats upstream failure as unavailable rather than current
- records the server observation timestamp
- exposes only public repository metadata
- stores no GitHub access token

## Security

The endpoint is fixed in source code. No user-supplied URL is accepted, preventing this module from becoming a generic SSRF proxy.

The module does not access private GitHub resources.

## Reliability

The browser refreshes the monitor every 30 seconds while the page is open. This is suitable for the initial single-user environment. Server-side caching or event-driven updates should be considered before the module is expanded to many users or many repositories.

## Future

A mature resource registry may represent many repositories and other digital assets. Such integrations should declare:
- source
- credential requirement
- polling/event strategy
- freshness policy
- failure semantics
- data classification
- rate-limit behavior
