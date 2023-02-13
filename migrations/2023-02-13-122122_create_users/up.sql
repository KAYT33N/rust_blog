-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username varchar NOT NULL,
    password char(64) NOT NULL
);