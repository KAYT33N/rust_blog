-- Your SQL goes here
CREATE TABLE access_tokens(
    id SERIAL PRIMARY KEY,
    hashed char(64) NOT NULL,
    user_id INT NOT NULL,
    age INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);