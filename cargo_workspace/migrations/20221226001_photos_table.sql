CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE photos (
    photo_id uuid not null,
    created_at int8 not null,
    photo_data bytea not null,
    PRIMARY KEY (photo_id)
);
