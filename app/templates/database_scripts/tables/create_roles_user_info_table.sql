CREATE TABLE IF NOT EXISTS roles_user_infos (
    id VARCHAR(36) PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    role_id VARCHAR(36) NOT NULL,
    params TEXT NOT NULL
);