CREATE TABLE IF NOT EXISTS account_sessions (
    id VARCHAR(36) PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    access_token VARCHAR(256) NOT NULL,
    refresh_token VARCHAR(256) NOT NULL,
    start_date DATE NOT NULL,
    access_token_creation_date DATE NOT NULL,
    refresh_token_creation_date DATE NOT NULL,
    last_usage_date DATE NOT NULL
);