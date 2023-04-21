CREATE TABLE
  users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    created timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted timestamptz,
    email text UNIQUE NOT NULL,
    role text NOT NULL,
    sub text UNIQUE NOT NULL,
    first_name text NOT NULL DEFAULT '',
    last_name text NOT NULL DEFAULT '',
    phone text NOT NULL DEFAULT '',
    avatar text NOT NULL DEFAULT '',
  );

CREATE TRIGGER set_timestamp BEFORE
UPDATE
  ON users FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp ();
