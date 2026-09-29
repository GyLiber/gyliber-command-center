# GyLiber Data Classification

Every future Command Center data object must receive a classification before persistence.

| Class | Example | Default handling |
|---|---|---|
| Public | published website content, public GitHub link | public display allowed |
| Internal | non-public operating notes, project metadata | authenticated members |
| Confidential | contracts, pricing, strategic documents | role-limited access + audit |
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

## v0.1 boundary

v0.1 is intentionally limited to Public/low-risk Internal demonstration state. Restricted and Critical GyLiber information remains gated.
