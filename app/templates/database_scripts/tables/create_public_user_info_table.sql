CREATE TABLE IF NOT EXISTS public_user_infos (
    id VARCHAR(36) PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    nickname VARCHAR(128) NOT NULL,
    info TEXT NOT NULL
);