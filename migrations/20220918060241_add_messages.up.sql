-- Add up migration script here
CREATE TABLE IF NOT EXISTS messages (
    id serial PRIMARY KEY,
    user_id INT REFERENCES users(id),
    created_at TIMESTAMP,
    content TEXT
);
