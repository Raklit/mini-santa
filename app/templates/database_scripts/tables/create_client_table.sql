CREATE TABLE IF NOT EXISTS clients (
    id VARCHAR(36) PRIMARY KEY,
    client_name VARCHAR(512) UNIQUE NOT NULL,
    password_hash VARCHAR(256) NOT NULL,
    password_salt VARCHAR(256) NOT NULL
);