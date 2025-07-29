-- Initial table creation
CREATE TABLE config_records (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    anthropic_base_url TEXT NOT NULL,
    anthropic_api_key TEXT,
    anthropic_auth_token TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);