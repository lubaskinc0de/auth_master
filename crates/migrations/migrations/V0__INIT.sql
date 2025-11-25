CREATE TABLE users (
    id UUID PRIMARY KEY,
    is_banned BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE
);

CREATE TYPE external_id_source AS ENUM ('WEB');

CREATE TABLE external_user_id (
    user_id UUID NOT NULL,
    external_id TEXT NOT NULL,
    source external_id_source NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (user_id, external_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_external_user_id_external_id ON external_user_id(external_id);