-- Add migration script here
CREATE TABLE users
(
    id         BIGSERIAL NOT NULL PRIMARY KEY,
    name       VARCHAR(255),
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);