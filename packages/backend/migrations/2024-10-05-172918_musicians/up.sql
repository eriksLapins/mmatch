-- Your SQL goes here
CREATE TABLE musicians (
    id TEXT PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL UNIQUE,
    stage_name VARCHAR(255) NOT NULL,
    bands JSON[] NOT NULL check (array_position(bands, null) is null),
    managers JSON[] check (array_position(managers, null) is null),
    links TEXT[] NOT NULL check (array_position(links, null) is null),
    skills TEXT[] NOT NULL check (array_position(skills, null) is null),
    open_to_collab_with TEXT[] NOT NULL check (array_position(open_to_collab_with, null) is null)
);