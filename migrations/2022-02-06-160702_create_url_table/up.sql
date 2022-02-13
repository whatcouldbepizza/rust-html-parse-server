-- Your SQL goes here
CREATE TABLE IF NOT EXISTS urls (
    id SERIAL PRIMARY KEY,
    url VARCHAR NOT NULL,
    result TEXT,
    status VARCHAR NOT NULL
);