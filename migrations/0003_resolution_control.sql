-- Private owner-scoped control metadata. No source documents or credentials.
CREATE TABLE IF NOT EXISTS gyliber_resolution_workspaces (
    owner TEXT PRIMARY KEY,
    generation TEXT NOT NULL,
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision BETWEEN 0 AND 2048),
    payload_bytes BIGINT NOT NULL DEFAULT 0 CHECK (payload_bytes BETWEEN 0 AND 3145728)
);
CREATE TABLE IF NOT EXISTS gyliber_resolution_events (
    owner TEXT NOT NULL REFERENCES gyliber_resolution_workspaces(owner),
    revision BIGINT NOT NULL CHECK (revision BETWEEN 1 AND 2048),
    document TEXT NOT NULL,
    PRIMARY KEY (owner, revision)
);
CREATE TABLE IF NOT EXISTS gyliber_resolution_operations (
    owner TEXT NOT NULL REFERENCES gyliber_resolution_workspaces(owner),
    operation_id TEXT NOT NULL,
    request_digest TEXT NOT NULL,
    result TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (owner, operation_id)
);
CREATE INDEX IF NOT EXISTS gyliber_resolution_operations_age ON gyliber_resolution_operations(created_at);
-- A purge changes the generation and invalidates backups from the old one.
CREATE TABLE IF NOT EXISTS gyliber_resolution_deletions (
    owner TEXT NOT NULL REFERENCES gyliber_resolution_workspaces(owner),
    generation TEXT NOT NULL,
    deleted_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (owner, generation)
);
CREATE TABLE IF NOT EXISTS gyliber_resolution_sources (
    owner TEXT NOT NULL REFERENCES gyliber_resolution_workspaces(owner),
    generation TEXT NOT NULL,
    PRIMARY KEY (owner, generation)
);
-- Denials and administrative operations are counted in one-minute buckets.
-- No request bodies, record titles, secrets, IP addresses or user-agent strings.
CREATE TABLE IF NOT EXISTS gyliber_resolution_audit (
    actor TEXT NOT NULL,
    event_code TEXT NOT NULL,
    minute TIMESTAMPTZ NOT NULL,
    occurrences BIGINT NOT NULL DEFAULT 1,
    PRIMARY KEY (actor, event_code, minute)
);
CREATE INDEX IF NOT EXISTS gyliber_resolution_audit_age ON gyliber_resolution_audit(minute);
