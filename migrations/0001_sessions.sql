CREATE TABLE IF NOT EXISTS gyliber_sessions (
    id TEXT PRIMARY KEY,
    data BYTEA NOT NULL,
    expiry_date TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS gyliber_sessions_expiry_idx
    ON gyliber_sessions (expiry_date);
