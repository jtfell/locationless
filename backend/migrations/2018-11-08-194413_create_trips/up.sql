CREATE TABLE IF NOT EXISTS trips (
    id serial PRIMARY KEY,
    "user" integer NOT NULL,
    city integer NOT NULL,
    start_date date NOT NULL,
    end_date date NOT NULL,

    created timestamp WITH time zone NOT NULL DEFAULT now(),
    updated timestamp WITH time zone NOT NULL DEFAULT now(),

    FOREIGN KEY (city) references cities (id),
    FOREIGN KEY ("user") references users (id)
);

ALTER SEQUENCE trips_id_seq restart 1000;

CREATE TRIGGER trips_updated BEFORE INSERT OR UPDATE ON trips
FOR EACH ROW EXECUTE PROCEDURE update_timestamp();
