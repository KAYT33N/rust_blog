-- Your SQL goes here
CREATE TABLE posts(
    id SERIAL PRIMARY KEY,
    parent_id INT NOT NULL,
    user_id INT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);