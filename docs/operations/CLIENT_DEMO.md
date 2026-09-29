# Client Demonstration Baseline

## Purpose

The Command Center is now developed as a live product. Each verified production change should become externally visible through the client-accessible application.

## Release loop

```
branch
  ↓
PR
  ↓
CI + security
  ↓
merge to main
  ↓
Render deployment
  ↓
live smoke test
  ↓
client-visible progress
```

## Demonstration evidence

Before a client progress meeting, verify:

1. The production URL opens.
2. `/api/health` reports `status=ok`.
3. The visible release matches the intended source revision.
4. The authenticated Command Center loads.
5. Live-state information is updating.
6. Recently completed capabilities are demonstrated from the running application.
7. Known limitations are stated explicitly.
8. The next coherent development target is identified.

## Build identity

Render supplies `RENDER_GIT_COMMIT` and `RENDER_GIT_BRANCH` at runtime. The authenticated Command Center surfaces these values through the protected live-state API so a deployed build can be traced to its source revision.

The public surface exposes only the minimum health contract.

## Environment boundary

The initial Render environment remains a low-risk engineering/demo environment. It is not an authorization to introduce critical company information, production credentials, or irreplaceable archives.
