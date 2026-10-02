-- Low-risk member coursework derivatives only; raw uploads are never stored.
CREATE TABLE IF NOT EXISTS gyliber_math_exhibits (
    id TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    document TEXT NOT NULL,
    engine TEXT NOT NULL,
    renderer TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ,
    repository_commit TEXT
);
CREATE INDEX IF NOT EXISTS gyliber_math_owner ON gyliber_math_exhibits (owner, created_at DESC);

-- Minimal durable action records also enforce the rolling 24-hour AI request budget.
CREATE TABLE IF NOT EXISTS gyliber_math_events (
    id BIGSERIAL PRIMARY KEY,
    owner TEXT NOT NULL,
    exhibit_id TEXT,
    event_code TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS gyliber_math_budget ON gyliber_math_events (owner, occurred_at);
