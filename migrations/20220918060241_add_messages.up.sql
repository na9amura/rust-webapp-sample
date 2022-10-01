-- Add up migration script here
CREATE TABLE IF NOT EXISTS messages (
    id serial PRIMARY KEY,
    user_id INT REFERENCES users(id) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    content TEXT NOT NULL
);
