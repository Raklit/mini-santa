CREATE TABLE IF NOT EXISTS account_sessions (
    id VARCHAR(36) PRIMARY KEY,
    account_id VARCHAR(36) PRIMARY KEY,
    auth_token VARCHAR(256) NOT NULL,
    refresh_token VARCHAR(256) NOT NULL,
    is_active BOOLEAN NOT NULL,
    is_ended BOOLEAN NOT NULL,
    start_date DATE NOT NULL,
    auth_token_creation_date DATE NOT NULL,
    refresh_token_creation_date DATE NOT NULL,
    last_usage_date DATE NOT NULL
);