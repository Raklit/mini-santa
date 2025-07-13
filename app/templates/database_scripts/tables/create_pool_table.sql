CREATE TABLE IF NOT EXISTS pools (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    account_id VARCHAR(36) NOT NULL,
    min_price INTEGER NOT NULL,
    max_price INTEGER NOT NULL,
    is_creator_involved BOOLEAN NOT NULL,
    creation_date DATE NOT NULL,
    lifetime INTEGER NOT NULL,
    state INTEGER NOT NULL
);