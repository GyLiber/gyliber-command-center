# Production-container verification

## Purpose

PR #22 addresses a hosted build failure that ordinary Rust source checks missed: the Docker build did not copy the SQL file embedded by `include_str!`. The `Production container` CI job now builds the actual Dockerfile and starts its final image against disposable PostgreSQL 18.

Production builds use the committed lockfile and `cargo build --release --locked`. The Rust image version must remain aligned with `rust-toolchain.toml` and the Rust CI job when the toolchain is intentionally updated.

## Run locally

Prerequisites: a running Docker engine, Bash, curl, jq and OpenSSL. Use a disposable test environment; no production configuration is needed.

```sh
docker build --tag gyliber-command-center:ci .
bash scripts/verify-container.sh gyliber-command-center:ci
```

The script creates uniquely named containers and a network, generates temporary test credentials, binds the application to a random loopback port, and cleans up containers, volumes, network and temporary files on exit. PostgreSQL is not published on a host port.

## Acceptance evidence

- Production exits unsuccessfully with the expected error when `DATABASE_URL` is missing.
- The final image runs as `appuser`.
- Startup creates the session schema from an empty database without manual SQL.
- `/api/health` returns the expected service identity, manifest release version and `status=ok`.
- Public landing HTML and packaged CSS are accessible.
- Anonymous Command Center access redirects and protected APIs return HTTP 401.
- OAuth start with synthetic credentials redirects without contacting GitHub and creates a database session record.
- Restart succeeds with the existing migration/schema and preserves that record's identity.

Failures propagate as a nonzero script exit and fail the CI job. Waits and HTTP requests are bounded. Existing Rust tests and security checks remain separate gates.

## Scope limits

This is container integration evidence, not hosted release evidence. It does not complete live OAuth, an allowlisted login, logout/revocation, persistence failure after startup, session expiry/collision behavior, backup/recovery or authenticated-cookie continuity across restart. Follow `DEPLOYMENT.md` for hosted acceptance and source-revision evidence.
