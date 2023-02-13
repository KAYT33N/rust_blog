-- Your SQL goes here
CREATE TABLE access_tokens(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    hashed char(64) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);