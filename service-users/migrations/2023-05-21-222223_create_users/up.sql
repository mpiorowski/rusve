CREATE TABLE
  users (
    id bytea PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted timestamptz,
    email text UNIQUE NOT NULL,
    role text NOT NULL,
    sub text UNIQUE NOT NULL,
    name text NOT NULL DEFAULT '',
    avatar_id uuid,
    payment_id text NOT NULL DEFAULT ''
  );

CREATE TRIGGER set_timestamp BEFORE
UPDATE
  ON users FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp ();
