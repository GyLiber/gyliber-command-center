# Manual Setup Runbook — v0.1.0

This document is for Gyile/GyLiber actions that cannot be performed automatically from the repository.

## 1. Cargo / Rust

### What is Cargo?

Cargo is Rust's build, dependency and test tool. It is **not an online service and does not require an account**.

### Does Gyile need to install it?

**For GitHub CI:** no. The repository's CI installs the pinned Rust toolchain and Cargo automatically.

**For local development:** yes, installing Rust via the official Rust installer is recommended. Installing Rust with rustup also installs Cargo.

### Ubuntu / Linux installation

Run these commands in a terminal:

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

Do **not** run `sudo apt install cargo` for this project unless the project documentation explicitly changes the toolchain strategy.

## 2. GitHub OAuth App

### What is it?

The Command Center must know which GitHub account a person is using without storing that person's GitHub password.

A GitHub OAuth App is the registration that lets GyLiber redirect a member to GitHub for authentication and receive the authentication result back. GitHub recommends the web authorization-code flow for browser applications and supports PKCE for protecting that flow.

The Command Center additionally checks an explicit GyLiber member allowlist. Possessing a GitHub account alone is not enough to enter the private hub.

### Create the app

1. Sign in to the GitHub account that owns the GyLiber repository.
2. Open GitHub.
3. Click the profile picture in the upper-right.
4. Click **Settings**.
5. In the left sidebar, click **Developer settings**.
6. Click **OAuth Apps**.
7. Click **New OAuth App**.

GitHub's current documented navigation is Settings → Developer settings → OAuth Apps → New OAuth App. Do not create a GitHub App for this v0.1 boundary unless the architecture document is changed first.

### Enter these values

**Application name**

`GyLiber Command Center`

**Homepage URL**

For local development:

`http://localhost:3000`

**Application description**

Use:

`GyLiber internal command-center authentication for authorized members.`

**Authorization callback URL**

For local development:

`http://localhost:3000/auth/github/callback`

Leave **Enable Device Flow** disabled.

Leave **Expire user access tokens** enabled. The v0.1 service uses the GitHub token only to obtain the authenticated identity and does not persist the GitHub access token.

Click **Register application**.

### Client ID and Client Secret

After registration GitHub displays the application's **Client ID**.

A **Client Secret** can be generated on the application page.

Treat the Client Secret exactly like a password for an infrastructure service:

- do not put it in GitHub source files;
- do not put it in `.env.example`;
- do not put it in a GitHub issue;
- do not paste it into ChatGPT;
- do not take a screenshot showing it.

Store it only in the local environment or hosting-provider secret store.

### Local configuration

Copy the repository's `.env.example` to a local `.env` file:

```bash
cp .env.example .env
```

Open `.env` in a text editor and replace only the placeholder values.

For `GYLIBER_ALLOWED_GITHUB_LOGINS`, enter the GitHub username(s) that GyLiber has explicitly authorized, separated by commas.

Example:

```text
GYLIBER_ALLOWED_GITHUB_LOGINS=GyLiber
```

Generate a local session key with:

```bash
openssl rand -base64 64
```

Copy the output into `SESSION_MASTER_KEY`.

For local testing keep:

```text
COOKIE_SECURE=false
```

because local HTTP is not HTTPS.

Do not commit the `.env` file.

## 3. Production OAuth callback

The production callback URL cannot be invented before the production hosting service gives GyLiber its real HTTPS hostname.

After deployment produces the production URL, the callback must be registered as the exact production URL followed by:

`/auth/github/callback`

Example shape only:

`https://YOUR-ACTUAL-HOST.example/auth/github/callback`

GitHub documents that callback URLs must match the configured URL and warns against unnecessary wildcard callback matching. Disable wildcard matching unless GyLiber has an explicit reason and control strategy for it.

## 4. Render account

The repository contains `render.yaml` for the initial free hosting route.

Before production deployment, Gyile must create/sign into a Render account and connect the GitHub account/repository.

The deployment runbook must then be followed to populate the required environment variables in Render. Member authentication is disabled until the GitHub OAuth credentials are configured; the public product may run safely without them.

Never put the production Client Secret or session key into a committed file.

## 5. What Gyile should do now

Do these actions in order:

1. Install Rust/Cargo locally using Section 1.
2. Run `rustc --version` and `cargo --version`.
3. Create the GitHub OAuth App using Section 2.
4. Do **not** paste the Client Secret into this chat.
5. Report the CI failure's exact GitHub Actions error text back to GyLiber engineering if the pipeline still fails after the current code correction.
6. Do not add real banking, personnel, trade-secret or irreplaceable company information to the v0.1 system.

## Safety rule

If a screen asks for a value that is not described in this document, stop at that field rather than guessing. The engineering baseline should be updated first.
