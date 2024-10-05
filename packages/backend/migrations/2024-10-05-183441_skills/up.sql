-- Your SQL goes here
CREATE TABLE skills (
    id TEXT PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    year_from VARCHAR(125) NOT NULL,
    year_to VARCHAR(125) NOT NULL,
    name VARCHAR(125) NOT NULL,
    level SMALLINT NOT NULL
);

CREATE TABLE year_from_to (
    id TEXT PRIMARY KEY,
    user_band_manager VARCHAR(255) NOT NULL,
    year_from VARCHAR(125) NOT NULL,
    year_to VARCHAR(125) NOT NULL,
    item JSON NOT NULL
);