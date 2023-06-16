-- +migrate Up
CREATE TABLE
  files (
    id bytea PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted timestamptz,
    target_id bytea NOT NULL,
    name text NOT NULL,
    type text NOT NULL
  );

CREATE TRIGGER set_timestamp BEFORE
UPDATE
  ON files FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp ();

