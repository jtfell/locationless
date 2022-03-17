CREATE TABLE IF NOT EXISTS friendships (
    user_a integer NOT NULL,
    user_b integer NOT NULL,
    user_a_accepted boolean NOT NULL,
    user_b_accepted boolean NOT NULL,

    -- Encodes when the friend request was created
    created timestamp with time zone not null default now(),
    -- Encodes when the friend request was accepted
    updated timestamp with time zone not null default now(),

    PRIMARY KEY(user_a, user_b),

    FOREIGN KEY(user_a) REFERENCES users(id),
    FOREIGN KEY(user_b) REFERENCES users(id),

    -- user_a is the user with the LOWER ID
    CONSTRAINT ck_user_order CHECK (user_a < user_b)
);

CREATE TRIGGER friendship_updated BEFORE INSERT OR UPDATE ON friendships
FOR EACH ROW EXECUTE PROCEDURE update_timestamp();
