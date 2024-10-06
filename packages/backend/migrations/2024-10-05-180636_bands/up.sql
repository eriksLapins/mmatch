-- Your SQL goes here
CREATE TABLE bands (
    id TEXT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    established_in INTEGER NOT NULL,
    description TEXT NOT NULL,
    country_of_origin VARCHAR(255) NOT NULL,
    members JSON[] NOT NULL check (array_position(members, null) is null),
    music_styles TEXT[] NOT NULL  check (array_position(music_styles, null) is null),
    instruments TEXT[] NOT NULL  check (array_position(instruments, null) is null),
    links TEXT[] NOT NULL  check (array_position(links, null) is null),
    managers JSON[] check (array_position(managers, null) is null),
    searching_for TEXT[] NOT NULL  check (array_position(searching_for, null) is null)
);