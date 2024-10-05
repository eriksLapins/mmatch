-- Your SQL goes here
CREATE TABLE managers (
    id TEXT PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    stage_name VARCHAR(255) NOT NULL,
    commission FLOAT ARRAY NOT NULL check (array_position(commission, null) is null),
    bands JSON ARRAY NOT NULL check (array_position(bands, null) is null),
    categories_interested_in TEXT ARRAY NOT NULL check (array_position(categories_interested_in, null) is null)
);