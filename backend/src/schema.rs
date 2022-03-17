table! {
    cities (id) {
        id -> Int4,
        name -> Text,
        latitude -> Float8,
        longitude -> Float8,
        country_code -> Text,
        continent_code -> Text,
        population -> Int4,
    }
}

table! {
    friendships (user_a, user_b) {
        user_a -> Int4,
        user_b -> Int4,
        user_a_accepted -> Bool,
        user_b_accepted -> Bool,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

table! {
    trips (id) {
        id -> Int4,
        user -> Int4,
        city -> Int4,
        start_date -> Date,
        end_date -> Date,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        username -> Nullable<Text>,
        photo_url -> Nullable<Text>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

table! {
    matches (trip_a, trip_b) {
        trip_a -> Int4,
        trip_b -> Int4,
    }
}

joinable!(trips -> cities (city));
joinable!(trips -> users (user));

allow_tables_to_appear_in_same_query!(
    cities,
    friendships,
    matches,
    trips,
    users,
);
