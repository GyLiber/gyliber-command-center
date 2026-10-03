# Manual Setup Runbook

This document covers actions that require Gyile/GyLiber because they involve an external account, secret, domain or approval boundary.

## 1. Cargo / Rust

### What is Cargo?

Cargo is Rust's build, dependency and test tool. It is **not an online service and does not require an account**.

### Does Gyile need to install it?

**For GitHub CI:** no. The repository's CI installs the pinned Rust toolchain and Cargo automatically.

**For local development:** yes, installing Rust via the official Rust installer is recommended. Installing Rust with rustup also installs Cargo.

### Ubuntu / Linux installation

Run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When the installer asks what to do, choose the default installation option.

Then load Cargo into the current terminal:

```bash
source "$HOME/.cargo/env"
```

Check the installation:

```bash
rustc --version
cargo --version
```

From the repository directory:

```bash
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

The repository contains `rust-toolchain.toml`, so rustup should use the project-pinned Rust version.

Do **not** run `sudo apt install cargo` for this project unless the toolchain strategy is explicitly changed.

## 2. GitHub OAuth App

### What is it?

The Command Center uses GitHub for member identity. A GitHub OAuth App lets the service redirect a member to GitHub for authentication without storing that person's GitHub password.

The Command Center additionally checks an explicit GyLiber member allowlist. A GitHub account by itself is not sufficient to enter the private hub.

### Production values

The current canonical-domain strategy means the production callback will eventually be:

`https://gyliber.com/auth/github/callback`

or, if the fallback corporate subdomain is used:

`https://hq.gyliber.com/auth/github/callback`

Until the canonical domain is actually bound, keep the current live callback configured by the deployment runbook.

### Secret handling

Treat the Client Secret exactly like a password for an infrastructure service:

- do not put it in GitHub source files;
- do not put it in `.env.example`;
- do not put it in a GitHub issue;
- do not paste it into ChatGPT;
- do not take a screenshot showing it.

Store it only in the local environment or hosting-provider secret store.

## 3. Canonical public domain — deferred

The canonical GyLiber-owned domain remains the preferred long-term public identity:

- preferred: `gyliber.com`
- fallback Command Center address: `hq.gyliber.com`

**Domain purchase is currently deferred for budget reasons.** The Command Center therefore continues to use the current managed-host URL as its temporary public address. No DNS or OAuth callback migration should be attempted until a GyLiber-owned domain is actually secured.

The long-term cutover procedure remains documented in `docs/operations/DEPLOYMENT.md`.

## 4. PostgreSQL session persistence

The PostgreSQL session implementation replaces production in-memory sessions.

### Required production value

Production requires:

`DATABASE_URL`

The value must be the private/internal PostgreSQL connection string for the managed deployment. Store it only in the hosting provider's environment/secret configuration.

Do not:

- commit the value to Git;
- put real credentials in `.env.example`;
- paste it into ChatGPT;
- place it in an issue, README or screenshot.

### Current development database

A free Render PostgreSQL 18 instance named `gyliber-command-center-db` is currently provisioned in Frankfurt for development verification. Render currently reports an expiry date of **2026-10-30**.

It is **temporary** and must not become the authoritative company archive or backup system. Real client contracts, credentials, critical company records or irreplaceable archives must not be loaded into this temporary database.

### Current configuration status

The private/internal `DATABASE_URL` has now been configured in the Render `gyliber-command-center` web service environment.

The value is intentionally not recorded in source control, documentation or this chat.

### What the application does

The application applies the versioned PostgreSQL session migration at startup when `DATABASE_URL` is configured. No manual SQL table creation is required.

### Deployment status

The durable-session code is merged into `main`, but the first controlled Render deployment exposed a Docker build-context defect: the image did not include the repository's `migrations/` directory.

PR #22 corrects the Dockerfile. Production activation therefore remains pending successful CI/security verification, merge and deployment of PR #22.

CI uses an ephemeral PostgreSQL service for automated session tests, so source verification does not depend on exposing the production database credential.

## 5. Current infrastructure secrets

The managed-host environment already contains the required session key and operational configuration.

Do not replace existing production values merely to “clean up” the environment.

Do not paste any secret value into chat.

## 6. Release rule

If a screen asks for a value not described by the current runbook, stop rather than guessing. Update the engineering documentation first.

## 7. What Gyile should do now

No manual action is required for ordinary source-code development, documentation updates, CI verification or routine deployment triggering when the engineering tooling can perform them.

The `DATABASE_URL` manual configuration has been completed. The canonical domain remains a separate deferred task.

No further manual database action is required for the current durable-session implementation.
