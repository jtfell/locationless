//  src/users/models.rs
//
//  Implements a basic User model

// https://github.com/rust-lang/rust/issues/50504
#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::web;
use chrono;
use diesel;
use diesel::prelude::*;
use validator::Validate;
use State;

use schema::users;
use util::database::{concat, least, levenshtein, lower};

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[table_name = "users"]
pub struct UserPopulated {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,

    // Derived fields from "friendships table"
    pub is_friend: bool,             // both users accepted
    pub is_pending_friend: bool,     // they have sent a friend request
    pub have_requested_friend: bool, // you have sent a friend request

    // Derived field from session info
    pub is_you: bool,
}

//
// Telegram User object
//
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Validate, Deserialize, Serialize, Debug)]
#[table_name = "users"]
pub struct NewOrExistingUser {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
}

pub fn new_or_existing(
    pl: NewOrExistingUser,
    data: web::Data<State>,
) -> Result<User, diesel::result::Error> {
    use schema::users::dsl::*;
    let conn = data.pool.get().unwrap();

    // Find the user in the DB
    diesel::update(users.filter(id.eq(pl.id)))
        .set(&pl)
        .get_result::<User>(&conn)
        // Or create the user if it doesn't exist yet
        .or_else(|_err| {
            diesel::insert_into(users)
                .values(&pl)
                .get_result::<User>(&conn)
        })
}

#[derive(Deserialize, Debug)]
pub struct UserLookup {
    pub id: i32,
    pub active_user_id: i32,
}

pub fn lookup(
    pl: UserLookup,
    data: web::Data<State>,
) -> Result<UserPopulated, diesel::result::Error> {
    use schema::users::dsl::*;
    let conn = data.pool.get().unwrap();

    users
        .filter(id.eq(pl.id))
        .get_result::<User>(&conn)
        .map(move |user| {
            let is_you = pl.id == pl.active_user_id;
            let is_friend = !is_you;
            let is_pending_friend = !is_you;
            let have_requested_friend = !is_you;

            UserPopulated {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                username: user.username,
                photo_url: user.photo_url,
                created: user.created,
                updated: user.updated,

                // Derived fields from "friendships table"
                is_friend,
                is_pending_friend,
                have_requested_friend,

                // Derived field from session info
                is_you: (pl.id == pl.active_user_id),
            }
        })
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct UserSearch {
    pub active_user_id: i32,
    pub q: String,
}

pub fn search(pl: UserSearch, data: web::Data<State>) -> Result<Vec<User>, diesel::result::Error> {
    use schema::users::dsl::*;
    let conn = data.pool.get().unwrap();

    //
    // Arbitrary limit on how far a search term can be from a name. May need to be tuned.
    //
    let string_distance_threshold = 6;

    //
    // Max number of results we return from a query
    //
    let max_results_per_query = 10;

    // Convert the input to lowercase (as we compare it against lowercase versions of user details)
    let query = format!("%{}%", pl.q.to_lowercase());

    //
    // Inspiration taken from here:
    //
    // http://tech.forwardfinancing.com/elixir/ecto/postgres/fuzzy-search/2017/12/20/fuzzy-search-in-elixir.html
    //
    // Keep an eye on this query, it will get VERY slow if there is a large
    // number of users in the DB. It's premature to optimise it now.
    //
    // Ideas:
    //  - Use a subquery to only compute levenshtein(lower(field), q) once per field we
    //    search on
    //
    // Uses the levenshtein function exported by the "fuzzystrmatch" psql extension.
    //
    // https://www.postgresql.org/docs/9.3/fuzzystrmatch.html
    users
        // rank results based on _one_ of the fields
        .order_by(least(
            levenshtein(lower(concat(first_name, " ", last_name)), query.clone()),
            levenshtein(lower(first_name), query.clone()),
            levenshtein(lower(last_name), query.clone()),
            levenshtein(lower(username), query.clone()),
        ))
        //  make sure at least one field is < threshold
        .filter(
            levenshtein(lower(concat(first_name, " ", last_name)), query.clone())
                .lt(string_distance_threshold)
                .or(levenshtein(lower(first_name), query.clone()).lt(string_distance_threshold))
                .or(levenshtein(lower(last_name), query.clone()).lt(string_distance_threshold))
                .or(levenshtein(lower(username), query.clone()).lt(string_distance_threshold)),
        )
        .limit(max_results_per_query)
        .get_results::<User>(&conn)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UsernameLookup {
    pub username: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPreview {
    pub id: i32,
    pub username: Option<String>,
    pub photo_url: Option<String>,
}

pub fn username_lookup(
    pl: UsernameLookup,
    data: web::Data<State>,
) -> Result<UserPreview, diesel::result::Error> {
    use schema::users::dsl::*;
    let conn = data.pool.get().unwrap();

    users
        .filter(username.eq(pl.username))
        .get_result::<User>(&conn)
        .map(move |user| UserPreview {
            id: user.id,
            username: user.username,
            photo_url: user.photo_url,
        })
}
