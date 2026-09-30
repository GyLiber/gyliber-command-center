# v0.3.0 Development Deployment Runbook

## 1. Deployment model

The application is deployed as a Dockerized Rust web service behind a managed hosting provider.

The hosting provider is an implementation detail. GyLiber's public identity must be represented by a GyLiber-owned canonical domain.

Current infrastructure is live on the initial managed-host environment. The provider-generated service hostname is retained for infrastructure operations only and must not be used as the business-card or canonical public address.

## 2. Required external dependency

Create a **GitHub OAuth App** before enabling live member authentication.

Runtime configuration:

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`
- `GITHUB_REDIRECT_URL`
- `SESSION_MASTER_KEY` — at least 64 cryptographically random bytes
- `DATABASE_URL` — private/internal PostgreSQL connection string in production
- `GYLIBER_ALLOWED_GITHUB_LOGINS`

The OAuth callback URL must exactly match the GitHub application configuration.

## 3. Security boundary

v0.2.0 is not authorized to store banking records, unrestricted financial data, high-value trade secrets, personnel records, production credentials or irreplaceable company archives.

## 4. Current release acceptance

A successful hosted deployment must demonstrate:

1. CI is green.
2. The container builds successfully.
3. The service starts on the managed host.
4. HTTPS is active.
5. `COOKIE_SECURE=true`.
6. GitHub OAuth callback URL is exact when member authentication is enabled.
7. Only intended GitHub usernames are allowlisted.
8. Production sessions use the PostgreSQL-backed session store.
9. `/api/health` reports `status=ok` and the expected release version.
10. The public landing page is reachable.
11. The authenticated Command Center is reachable for an allowlisted account when authentication is configured.
12. A non-allowlisted GitHub account receives HTTP 403.
13. Logout clears the authenticated session.
14. No sensitive company data has been loaded.
15. The authenticated Live State Monitor reports the release and running deployment provenance.
16. The canonical public domain resolves to the live service once domain binding is complete.

## 5. Canonical-domain binding

The public URL strategy is:

- preferred: `https://gyliber.com/`
- fallback: `https://hq.gyliber.com/` when the root domain is reserved for another public corporate surface

The domain must be owned and controlled by GyLiber.

When the domain is ready:

1. Add the canonical domain in the managed-host service's custom-domain settings.
2. Configure DNS with the registrar/DNS provider.
3. Remove conflicting `AAAA` records if the hosting provider requires IPv4-only routing.
4. Verify the domain and wait for managed TLS issuance.
5. Change the GitHub OAuth callback URL to the canonical HTTPS domain plus `/auth/github/callback`.
6. Change `GITHUB_REDIRECT_URL` in the hosting environment to the same canonical callback URL.
7. Test member authentication.
8. Make the canonical domain the only address printed on business cards and client-facing materials.
9. Keep the infrastructure hostname out of public branding.

The exact DNS record depends on the provider and the DNS host. The provider's custom-domain configuration supplies the authoritative target value at the time of binding.

## 6. Current hosted-environment verification

The initial production-like environment was successfully deployed after correcting the runtime image to include system CA certificates.

The deployment fix was necessary because the `debian:bookworm-slim` runtime image initially lacked system trust roots required by `rustls`.

The corrected image installs:

```text
ca-certificates
```

before the application starts.

## 7. Render/managed-host free-tier constraints

The current free PostgreSQL development instance expires on **2026-10-30** and is not approved as permanent storage or backup infrastructure.

## 8. Render/managed-host free-tier constraints

The initial managed-host environment is suitable for development, demonstration and low-risk operation, not for critical company data.

Treat the hosted filesystem as disposable unless a separate durable storage contract exists.

Therefore:

- do not use the free service as the authoritative company archive;
- do not use Free Postgres for critical records;
- treat every local filesystem write as disposable;
- keep source and infrastructure definitions in version control;
- establish independent backups before persistent high-value information is introduced.

## 9. Manual changes

Infrastructure credentials and external DNS changes are the only deployment actions that should require Gyile/GyLiber intervention.

Never put the production Client Secret or session key into a committed file.

## 10. Release evidence

For each material release, retain:

- source commit
- verification result
- deployment identifier
- canonical public URL
- live health result
- visible release/build identity
- known limitations
- next development target

A release is not considered demonstrated merely because a build succeeded.
