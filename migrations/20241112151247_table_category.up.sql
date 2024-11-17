-- Add up migration script here
CREATE TABLE IF NOT EXISTS "category" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);