# Manual Setup Runbook — v0.3.0 development

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

The v0.3 development line replaces production in-memory sessions with PostgreSQL-backed sessions.

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

### What Gyile must do before production activation

1. Open the Render PostgreSQL resource's **Connect** view.
2. Copy its private/internal database connection string.
3. Open the `gyliber-command-center` Web Service.
4. Open **Environment**.
5. Add an environment variable with key `DATABASE_URL`.
6. Paste the private/internal connection string as its value.
7. Save the environment changes.

The connection string itself must never be pasted into this chat.

CI uses an ephemeral PostgreSQL service for automated session tests, so the source verification does not depend on exposing the production database credential.

## 5. Current infrastructure secrets

The managed-host environment already contains the required session key and operational configuration.

Do not replace existing production values merely to “clean up” the environment.

Do not paste any secret value into chat.

## 6. Release rule

If a screen asks for a value not described by the current runbook, stop rather than guessing. Update the engineering documentation first.

## 7. What Gyile should do now

No manual action is required for ordinary source-code development, documentation updates, CI verification or routine deployment triggering when the engineering tooling can perform them.

The current manual deployment dependency is the **DATABASE_URL** configuration described above. The canonical domain remains a separate deferred task and does not block v0.3 development.
