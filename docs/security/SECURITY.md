# Security Engineering Baseline

## Threat model

Protect against:
- credential theft
- session compromise
- authorization bypass
- injection
- XSS/CSRF/SSRF
- dependency/supply-chain compromise
- exposed secrets
- destructive administrative actions
- cloud misconfiguration
- insider misuse
- accidental deletion
- hosting/storage loss
- phishing/social engineering
- unauthorized disclosure of GyLiber IP

## Core rules

1. The browser is hostile input.
2. Authentication is not authorization.
3. Secrets are not application data.
4. Logs must not contain passwords, tokens or unnecessary sensitive payloads.
5. High-value operations require explicit authorization and eventually step-up authentication.
6. Production secrets never enter Git history.
7. Security controls are tested, monitored and reviewed.
8. Sensitive data is minimized and isolated by trust boundary.
9. Backup copies must be independently recoverable.
10. AI-generated code receives the same verification as human code.

## Authentication progression

### v0.1
Provide the protected member gateway and a pluggable identity boundary. Deployment configuration must not silently fall back to insecure authentication.

### v1.x
Integrate a mature OIDC identity provider with allowlisted membership/claims and passkey/WebAuthn support.

### Later high-impact phase
Require phishing-resistant authentication for sensitive operations, strong recovery controls, session revocation, device/session visibility and administrative approval workflows.

## Authorization progression

Use deny-by-default policies based on:
```text
actor + resource + action + classification + policy
```

Never rely on hidden UI controls.

## Secure development

CI should ultimately enforce:
- cargo fmt/check/clippy
- tests
- RustSec/advisory audit
- dependency policy
- frontend dependency checks where present
- secret scanning
- license/provenance checks
- container scan where containers are built
- SBOM generation for release artifacts

## Incident response

The future operational runbook must define:
- credential revocation
- compromised-session handling
- service isolation
- evidence preservation
- notification/escalation
- restoration
- post-incident review

## Security claims

The project must describe tested scope and evidence. It must never claim immunity from compromise.
