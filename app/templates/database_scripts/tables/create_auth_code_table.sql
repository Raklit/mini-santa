CREATE TABLE IF NOT EXISTS auth_code (
    id VARCHAR(36) PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    code VARCHAR(256) NOT NULL,
    creation_date DATE NOT NULL
);