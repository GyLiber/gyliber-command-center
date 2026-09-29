# Requirements Baseline — GyLiber Command Center

**Client:** Gyile / GyLiber  
**Target architecture:** v1.0.0  
**First executable release:** v0.1.0  
**Current live baseline:** v0.2.0  
**Status:** Living baseline

## 1. Mission

The Command Center is the central digital hub for GyLiber's public identity, authenticated internal operations, resource representations, knowledge, live system state and future company information.

It must optimize **useful workflow output per unit of human attention**: high-value state should be visible quickly, stale/broken information should be obvious, and drill-down should be fast.

## 2. Product surfaces

### Public
- company/brand identity
- selected work/projects
- public resource links such as GitHub, Upwork and future accounts
- contact/inquiry capability as introduced
- discreet member login entry
- canonical GyLiber-owned public URL independent of the hosting provider

### Private
- authenticated command center
- operational/resource status
- future modules for company knowledge, assets, finances, staff, documents, integrations and contract/engagement records
- auditability and role-based access
- live or near-live information with freshness metadata

Future modules can remain in the same site, become separately deployed applications, or both; the Command Center is the navigation/control plane.

## 3. Public identity and hosting abstraction

The public product must not expose a provider-generated hosting hostname as its primary identity.

Requirements:

- GyLiber owns or controls the canonical public domain.
- Business cards and client-facing material use only the canonical domain.
- The canonical domain must remain stable if the underlying hosting provider changes.
- Infrastructure/provider URLs remain operational references, not brand addresses.
- Canonical HTTPS must be used for the public surface.
- Authentication callback URLs must exactly match the canonical production origin.

The preferred public presentation is `https://gyliber.com/`, subject to domain ownership and availability. If the root domain is reserved for another corporate surface, a short subdomain such as `https://hq.gyliber.com/` may be used.

## 4. Security requirements

Security starts with the first commit. No programming language makes a system unbreachable.

The baseline is:
- Rust for security-sensitive backend services
- no Python application dependency
- deny-by-default authorization
- least privilege
- secure session handling
- phishing-resistant authentication/passkeys in the mature identity design
- strong input validation and output encoding
- CSRF/XSS/SQLi/SSRF/path-traversal defenses
- security headers and HTTPS
- dependency/advisory scanning
- secret scanning
- SBOM/provenance for serious releases
- short-lived deployment credentials through OIDC where supported
- audited sensitive operations
- independent security review before high-impact data is onboarded

Use OWASP ASVS 5.0 and NIST SSDF as process/verification baselines.

## 5. Protected information

### Restricted / later-gated
- banking and financial account data
- payment credentials and API keys
- software trade secrets and proprietary methods
- staff/personnel records
- client/customer information
- contracts, legal material and invoices
- strategy, pricing and financial models
- copyright/licensing evidence
- external-service credentials
- critical backups and disaster-recovery material
- irreplaceable company knowledge
- active and planned client engagements, contract status, commercial terms and related agreement records

v0.1/v0.2 must not contain real high-value records. The Contracts & Engagements module is represented in the authenticated registry as a Confidential, security-gated capability only.

## 6. Availability and recovery

The laptop is never the only source of truth.

The eventual production design requires:
- source-controlled infrastructure/configuration
- durable object storage
- independent backup copies
- backup monitoring
- encryption for critical backups
- restore drills
- defined RPO/RTO by data class
- cross-provider/region strategy for irreplaceable information

A backup is not accepted as evidence until restoration has been demonstrated.

## 7. v0.1.0 acceptance

A deployed v0.1.0 slice must demonstrate:
1. public landing page
2. discreet member entry
3. protected command-center route
4. live backend health/state API
5. persistent-safe architecture boundary, without production-sensitive records
6. automated tests
7. CI on every relevant repository change
8. deployment configuration/runbook
9. security baseline and secret hygiene
10. version metadata

## 8. v0.1.x evolution

The v0.1.x line may harden the transport boundary, extract domain modules, establish module-registry contracts, and improve operational observability without introducing critical business data.

## 9. v0.2.0 live foundation

The current live foundation adds:
- runtime release/build provenance
- public live-health status
- protected resource/module surfaces
- explicit member authentication policy
- safe public bootstrap behavior
- managed-host deployment
- live demonstration evidence
- a provider-independent canonical-domain strategy

## 10. v1.0.0 expansion

The architecture must permit:
- resource registry and integrations
- event/audit timeline
- live operational panels
- creative knowledge/trivia modules
- documents/IP records
- contract and engagement records with role-limited access and durable auditability
- role and policy management
- secure secrets integration
- durable backups/recovery
- richer observability
- separate services/sites when necessary

## 11. Traceability

Every material feature after v0.1.0 should link to a documented requirement, implementation boundary and automated acceptance evidence.
