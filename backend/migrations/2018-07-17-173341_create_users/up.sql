CREATE OR REPLACE FUNCTION update_timestamp() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated = now(); 
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TABLE IF NOT EXISTS users (
    id integer unique PRIMARY KEY,
    first_name text,
    last_name text,
    username text unique,
    photo_url text,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

CREATE TRIGGER user_updated BEFORE INSERT OR UPDATE ON users
FOR EACH ROW EXECUTE PROCEDURE update_timestamp();
