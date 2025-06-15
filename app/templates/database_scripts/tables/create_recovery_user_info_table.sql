CREATE TABLE IF NOT EXISTS recovery_user_infos (
    id VARCHAR(36) PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL
);