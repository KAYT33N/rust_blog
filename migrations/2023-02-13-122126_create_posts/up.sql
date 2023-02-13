-- Your SQL goes here
CREATE TABLE posts(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);