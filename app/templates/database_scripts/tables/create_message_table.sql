CREATE TABLE IF NOT EXISTS messages (
    id VARCHAR(36) PRIMARY KEY,
    text_content TEXT NOT NULL,
    account_id VARCHAR(36) NOT NULL,
    room_id VARCHAR(36) NOT NULL,
    creation_date DATE NOT NULL
);