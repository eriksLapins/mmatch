-- Your SQL goes here
CREATE TABLE skills (
    id TEXT PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    year_from VARCHAR(125) NOT NULL,
    year_to VARCHAR(125) NOT NULL,
    name VARCHAR(125) NOT NULL,
    level SMALLINT NOT NULL
);

CREATE TABLE year_from_to (
    id TEXT PRIMARY KEY,
    year_from VARCHAR(125) NOT NULL,
    year_to VARCHAR(125) NOT NULL,
    item_type user_types NOT NULL,
    item_id VARCHAR(255) NOT NULL
);

CREATE TABLE musician_with_purpose (
    band_id VARCHAR(255) NOT NULL REFERENCES bands(id),
    musician_id VARCHAR(255) NOT NULL REFERENCES musicians(id),
    main_purpose VARCHAR(255) NOT NULL,
    PRIMARY KEY(band_id, musician_id)
)