# GyLiber Data Classification

Every future Command Center data object must receive a classification before persistence.

| Class | Example | Default handling |
|---|---|---|
| Public | published website content, public GitHub link | public display allowed |
| Internal | non-public operating notes, project metadata | authenticated members |
| Confidential | contracts, engagement records, pricing, strategic documents | role-limited access + audit |
| Restricted | personnel, customer records, financial details, sensitive IP | strong identity + narrow authorization + audit |
| Critical | bank credentials, master recovery material, irreplaceable archives | dedicated secret/storage controls + independent recovery + step-up authorization |

## Rules

1. Classification follows the information, not the page displaying it.
2. A UI hiding a value does not protect it.
3. Restricted and Critical information must not be introduced merely because a database exists.
4. Secrets such as passwords, API keys, private keys and recovery tokens belong in a secret-management system, not ordinary database fields.
5. Critical records require an independently recoverable backup strategy and tested restoration.
6. Logs must never duplicate Restricted/Critical payloads unnecessarily.
7. Retention and deletion rules must be defined before long-term archival.

## Contract & engagement records

Contract and engagement records are Confidential by default. The future module is intended to track active and planned engagements, agreement status, key dates, scope summaries, commercial terms, billing milestones and links to controlled agreement documents. More sensitive client, personnel, financial or credential material may require a higher classification.

The Command Center must not expose real contract records merely because the module exists. Activation requires durable sessions, resource-level authorization, durable audit events, protected persistence, appropriate document storage and tested backup/recovery controls. Public pages and unauthenticated APIs must never expose contract data.

## v0.1 boundary

v0.1 is intentionally limited to Public/low-risk Internal demonstration state. Restricted and Critical GyLiber information remains gated.
