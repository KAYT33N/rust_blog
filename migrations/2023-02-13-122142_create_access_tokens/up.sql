-- Your SQL goes here
CREATE TABLE access_tokens(
    hashed char(64) PRIMARY KEY,
    user_id INT NOT NULL,
    age INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);