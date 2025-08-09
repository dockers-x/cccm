-- Initial table creation with all fields
CREATE TABLE config_records (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    anthropic_base_url TEXT NOT NULL,
    anthropic_api_key TEXT,
    anthropic_auth_token TEXT,
    access_password TEXT,
    setup_method TEXT DEFAULT 'config_file',
    anthropic_model TEXT,
    anthropic_small_fast_model TEXT,
    anthropic_small_fast_model_aws_region TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);