# Manual Setup Runbook — v0.2.0

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

## 3. Canonical public domain

The public site should not expose a provider-generated hosting hostname in business-card or client-facing material.

### Target

Preferred:

`gyliber.com`

Preferred Command Center subdomain when the root domain is used elsewhere:

`hq.gyliber.com`

The final choice depends on which domain GyLiber owns and intends to make the public corporate address.

### What Gyile must do

1. Own or control the chosen domain through a registrar/DNS provider.
2. Add the chosen domain to the managed-host service's **Custom Domains** section.
3. Copy the exact DNS target shown by the hosting provider.
4. Add that DNS record at the domain's DNS provider.
5. Remove conflicting IPv6 `AAAA` records if instructed by the hosting provider.
6. Return to the hosting provider and click **Verify**.
7. Confirm that HTTPS/TLS becomes active.
8. Update the GitHub OAuth callback to the canonical domain.
9. Update `GITHUB_REDIRECT_URL` in the hosting environment to match exactly.
10. Test login and the public site using the canonical domain.
11. Use only the canonical domain on printed material.

The underlying hosting provider can then be changed later without changing the public address.

## 4. Current infrastructure secrets

The managed-host environment already contains the required session key and operational configuration.

Do not replace existing production values merely to “clean up” the environment.

Do not paste any secret value into chat.

## 5. Release rule

If a screen asks for a value not described by the current runbook, stop rather than guessing. Update the engineering documentation first.

## 6. What Gyile should do now

No manual action is required for ordinary source-code development, documentation updates, CI verification or routine deployment triggering when the engineering tooling can perform them.

The next manual dependency is the **GyLiber-owned canonical domain**. Once a domain is owned and selected, the remaining DNS/custom-domain/OAuth callback steps can be executed from this runbook.
