# GyLiber Command Center

> The operational command center for GyLiber's digital assets, systems, knowledge, and future operations.

## Status

**Release track:** v0.x → v1.0.0  
**Current milestone:** v0.1.0 foundation

This repository contains the security-first foundation of GyLiber's long-lived Command Center Hub. The platform is intended to become a central operational interface for public GyLiber information, authenticated internal resources, live system representations, knowledge modules, and future business capabilities.

## Engineering principles

- Security is designed from the first commit.
- Every commit represents a coherent, reviewable unit of work.
- Automated verification is part of development.
- Secrets never belong in source control.
- Sensitive production information is gated behind explicit readiness criteria.
- Public presentation and authenticated operations are separate trust zones.
- Architecture may evolve when implementation evidence invalidates an earlier assumption.
- AI-assisted work is reviewed and verified as engineering work; it is not treated as an authority.

## Technology direction

The application stack intentionally excludes Python. The current baseline is a Rust backend with a modern web frontend and PostgreSQL persistence. Dependency versions are pinned in implementation files and lockfiles.

## Repository structure

```text
docs/           Project requirements, architecture, security and governance
backend/        Rust API/backend
frontend/       Web client
.github/        CI/CD and repository automation
```

## Security boundary

v0.1.0 is a working end-to-end foundation, not a declaration that the platform is ready for unrestricted production secrets.

Until the relevant security, identity, backup, recovery, monitoring and operational gates are satisfied, do not place banking credentials, unrestricted financial data, high-value trade secrets, personnel records, or irreplaceable corporate archives in the system.

## Development

Use conventional commits, for example:

```text
chore: initialize repository structure
feat: add public command center shell
test: add authentication integration coverage
fix: reject expired sessions
```

The authoritative checks are run by CI from a clean environment.

## AI-assisted development

AI is used intentionally as an engineering accelerator for research, implementation, testing, documentation and refactoring. Human ownership remains responsible for requirements, architecture, security decisions, intellectual-property decisions, verification, releases and production operation.

## Intellectual property

Repository visibility during v0.x does not grant permission to reuse proprietary GyLiber materials. The project's formal licensing and IP policy will be established before protected company assets are published or distributed.

## License

License terms are pending formal GyLiber policy.
