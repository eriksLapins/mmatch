-- Your SQL goes here

CREATE TYPE user_types AS ENUM ('musician', 'manager', 'explorer');

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    name VARCHAR(125) NOT NULL,
    lastname VARCHAR(125) NOT NULL,
    description TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    phone VARCHAR(125) NOT NULL,
    phone_prefix VARCHAR(125) NOT NULL,
    country VARCHAR(125) NOT NULL,
    city VARCHAR(255) NOT NULL,
    street VARCHAR(255) NOT NULL,
    house_number VARCHAR(16) NOT NULL,
    apartment VARCHAR(16),
    types user_types ARRAY NOT NULL,
    UNIQUE(email)
)