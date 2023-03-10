-- Add down migration script here
CREATE TABLE
    users (
        id VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT (UUID()),
        name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        photo VARCHAR(255) NOT NULL DEFAULT 'default.png',
        verified BOOLEAN NOT NULL DEFAULT FALSE,
        password VARCHAR(100) NOT NULL,
        role VARCHAR(50) NOT NULL DEFAULT 'user',
        created_at TIMESTAMP DEFAULT NOW(),
        updated_at TIMESTAMP DEFAULT NOW()
    );

CREATE INDEX users_email_idx ON users (email);