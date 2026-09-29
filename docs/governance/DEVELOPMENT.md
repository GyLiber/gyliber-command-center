# Development & Contract Governance

## Client

Gyile / GyLiber

## Engineering model

This repository is operated as professional software development work. Requirements, implementation, testing, security, operations and releases are all part of the deliverable.

## Commit discipline

Use Conventional Commits.

Examples:
- `chore: initialize repository structure`
- `docs: establish security baseline`
- `feat: add public command center shell`
- `test: add protected route coverage`
- `fix: reject invalid sessions`
- `refactor: isolate resource status module`

A commit should leave the repository in a coherent state even when the complete project is unfinished.

Correct earlier work may be changed when new evidence makes it incorrect. Prefer a corrective/refactor commit that preserves history over hiding the change.

## Definition of done

For a material change:
- implementation exists
- applicable automated tests pass
- security implications are considered
- documentation is updated where necessary
- CI can reproduce the checks
- no secrets are introduced

## Public-repository rule

Until the repository becomes private, assume every committed byte is observable. Never store company secrets, customer information, private credentials, private keys, production database dumps or protected business records here.

## AI-assisted development

AI is an engineering accelerator for research, coding, testing and documentation. It does not replace human responsibility.

AI-assisted work must pass the same:
- review
- test
- dependency
- security
- IP/licensing
- release

requirements as human-authored work.

## Architectural change

When implementation evidence invalidates an existing architecture choice:
1. preserve the historical record;
2. document the reason;
3. make the smallest coherent correction;
4. run affected verification;
5. update design/requirements if the contract changed.

## Team scale

The architecture must allow approximately 6–10 developers to work concurrently through domain boundaries, ownership rules, stable interfaces, CI enforcement and review practices.
