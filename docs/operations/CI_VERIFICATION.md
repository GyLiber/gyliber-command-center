# CI and release verification

## Required external dependency

Create a **GitHub OAuth App** before enabling live member authentication.

Runtime configuration:

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`
- `GITHUB_REDIRECT_URL`
- `SESSION_MASTER_KEY` — at least 64 cryptographically random bytes
- `DATABASE_URL` — required for production session persistence
- `GYLIBER_ALLOWED_GITHUB_LOGINS`

The OAuth callback URL must exactly match the GitHub application configuration.

## Security boundary

The current development environment is not authorized to store banking records, unrestricted financial data, high-value trade secrets, personnel records, production credentials or irreplaceable company archives.

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

The early environment is for engineering/demo use. Introduce durable managed database/object storage and independent backup infrastructure before critical company information is onboarded. PostgreSQL session persistence alone does not satisfy these controls.


## CI verification policy

A release candidate is not considered verified until a clean GitHub Actions run completes successfully for the exact commit being released. Historical failed or cancelled runs remain part of the audit trail and do not invalidate a later successful run.

The CI workflow also builds the locked production Docker image and runs the `Production container` job against disposable PostgreSQL 18. See `CONTAINER_VERIFICATION.md` for acceptance scope and local reproduction. Source checks alone are insufficient to establish container packaging or startup correctness.
