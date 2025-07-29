CREATE TABLE IF NOT EXISTS members (
    id VARCHAR(36) PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    room_id VARCHAR(36) NOT NULL,
    pool_id VARCHAR(36) NOT NULL,
    wishlist TEXT NOT NULL
);