# v0.1.0 Deployment Runbook

## Required external dependency

Create a **GitHub OAuth App** before enabling live member authentication.

Runtime configuration:

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`
- `GITHUB_REDIRECT_URL`
- `SESSION_MASTER_KEY` — at least 64 cryptographically random bytes
- `GYLIBER_ALLOWED_GITHUB_LOGINS`

The OAuth callback URL must exactly match the GitHub application configuration.

## Security boundary

v0.1.0 is not authorized to store banking records, unrestricted financial data, high-value trade secrets, personnel records, production credentials or irreplaceable company archives.

## First deployment checks

1. CI is green.
2. HTTPS is active.
3. `COOKIE_SECURE=true`.
4. GitHub OAuth callback URL is exact.
5. Only intended GitHub usernames are allowlisted.
6. `/api/health` reports `status=ok`.
7. An allowlisted account can enter `/command`.
8. A non-allowlisted GitHub account receives HTTP 403.
9. Logout clears the authenticated session.
10. No sensitive company data has been loaded.

## Hosting evolution

The early environment is for engineering/demo use. Introduce durable managed database/object storage and independent backup infrastructure before critical company information is onboarded.


## CI verification policy

A release candidate is not considered verified until a clean GitHub Actions run completes successfully for the exact commit being released. Historical failed or cancelled runs remain part of the audit trail and do not invalidate a later successful run.
