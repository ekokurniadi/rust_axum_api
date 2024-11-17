-- Add up migration script here
CREATE TABLE IF NOT EXISTS "products" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    category_id integer NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);