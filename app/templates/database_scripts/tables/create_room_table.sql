CREATE TABLE IF NOT EXISTS rooms (
    id VARCHAR(36) PRIMARY KEY,
    pool_id VARCHAR(36) NOT NULL,
    mailer_id VARCHAR(36) NOT NULL,
    recipient_id VARCHAR(36) NOT NULL,
    room_state INTEGER NOT NULL
);