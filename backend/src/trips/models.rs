//  src/trips/models.rs
//
//  Implements a basic Trip model

// https://github.com/rust-lang/rust/issues/50504
#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::web;
use chrono;
use diesel;
use diesel::prelude::*;

use cities::models::City;
use schema::{cities, matches, trips, users};
use users::models::{User, UserPopulated};
use State;

//
// Core Trip Model
//

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[belongs_to(User, foreign_key = "user")]
#[belongs_to(City, foreign_key = "city")]
pub struct Trip {
    pub id: i32,
    pub user: i32,
    pub city: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

//
// Trip with User and City fields populated
//

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TripPopulated {
    pub id: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub user: User,
    pub city: City,
    pub matches: Vec<MatchingTrip>,
}

//
// Structs for interacting with trip Matches
//

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[primary_key(trip_a, trip_b)]
#[table_name = "matches"]
pub struct Match {
    pub trip_a: i32,
    pub trip_b: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MatchingTrip {
    pub user: UserPopulated,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

//
// POST /trips/ handler
//

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "trips"]
pub struct NewTrip {
    pub city: i32,
    pub user: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

pub fn new(msg: NewTrip, data: web::Data<State>) -> Result<Trip, diesel::result::Error> {
    use schema::trips::dsl::*;
    let conn = data.pool.get().unwrap();
    diesel::insert_into(trips)
        .values(&msg)
        .get_result::<Trip>(&conn)
}

//
// DELETE /trips/:id handler
//

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteTrip {
    pub id: i32,
    pub user: i32,
}

pub fn delete(msg: DeleteTrip, data: web::Data<State>) -> Result<(), diesel::result::Error> {
    use schema::trips::dsl::*;
    let conn = data.pool.get().unwrap();

    diesel::delete(trips.filter(id.eq(msg.id).and(user.eq(msg.user))))
        .execute(&conn)
        .map(|_| ())
}

//
// PUT /trips/:id handler
//

#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[table_name = "trips"]
pub struct UpdateTrip {
    pub id: i32,
    pub city: i32,
    pub user: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

pub fn update(msg: UpdateTrip, data: web::Data<State>) -> Result<Trip, diesel::result::Error> {
    use schema::trips::dsl::*;
    let conn = data.pool.get().unwrap();
    diesel::update(trips.filter(id.eq(msg.id).and(user.eq(msg.user))))
        .set(&msg)
        .get_result::<Trip>(&conn)
}

//
// GET /trips/{id} handler
//

#[derive(Serialize, Deserialize, Debug)]
pub struct TripLookup {
    pub id: i32,
    pub active_user_id: i32,
}

pub fn lookup(msg: TripLookup, data: web::Data<State>) -> Result<TripPopulated, diesel::result::Error> {
    use schema::trips::dsl::*;
    let conn = data.pool.get().unwrap();

    // Lookup the trip (joining the User Record)
    let db_lookup_res = trips
        .inner_join(users::table.on(user.eq(users::id).and(id.eq(msg.id))))
        .inner_join(cities::table.on(city.eq(cities::id)))
        .filter(user.eq(msg.active_user_id))
        .get_result::<(Trip, User, City)>(&conn);

    // Nest the results into the desired structure
    db_lookup_res.and_then(|(t, u, c)| {
        let db_lookup_res_matches = trips
            .inner_join(matches::table.on(id.eq(matches::trip_a).or(id.eq(matches::trip_b))))
            .inner_join(users::table.on(user.eq(users::id)))
            .filter(matches::trip_a.eq(msg.id))
            .or_filter(matches::trip_b.eq(msg.id))
            .get_results::<(Trip, Match, User)>(&conn);

        db_lookup_res_matches.map(|list_of_matches| {
            let matches = get_matches_for_trip(list_of_matches, msg.id, msg.active_user_id);

            TripPopulated {
                id: t.id,
                start_date: t.start_date,
                end_date: t.end_date,
                created: t.created,
                updated: t.updated,
                user: u,
                city: c,
                matches,
            }
        })
    })
}

#[derive(Deserialize, Debug)]
pub struct UserTripLookup {
    pub user: i32,
    pub end_date_after: chrono::NaiveDate,
}

pub fn user_trip_lookup(
    msg: UserTripLookup,
    data: web::Data<State>,
) -> Result<Vec<TripPopulated>, diesel::result::Error> {
    use schema::trips::dsl::*;
    let conn = data.pool.get().unwrap();

    // Get all the trips for the user (ending after today)
    let db_lookup_res_trips = trips
        .inner_join(users::table.on(user.eq(users::id)))
        .inner_join(cities::table.on(city.eq(cities::id)))
        .filter(user.eq(msg.user).and(end_date.gt(msg.end_date_after)))
        .get_results::<(Trip, User, City)>(&conn);

    db_lookup_res_trips.map(|list_of_trips| {
        let user_trip_ids = list_of_trips
            .clone()
            .into_iter()
            .map(|(t, _u, _c)| t.id)
            .collect::<Vec<_>>();

        // Get the matching trips from other users
        let db_lookup_res_matches = trips
            .inner_join(matches::table.on(id.eq(matches::trip_a).or(id.eq(matches::trip_b))))
            .inner_join(users::table.on(user.eq(users::id)))
            .filter(matches::trip_a.eq_any(user_trip_ids.clone()))
            .or_filter(matches::trip_b.eq_any(user_trip_ids.clone()))
            .get_results::<(Trip, Match, User)>(&conn);

        list_of_trips
            .into_iter()
            .map(move |(t, u, c)| {
                let matches = match &db_lookup_res_matches {
                    Ok(list_of_matches) => {
                        get_matches_for_trip(list_of_matches.clone(), t.id, msg.user)
                    }
                    Err(_e) => Vec::new(),
                };

                TripPopulated {
                    id: t.id,
                    start_date: t.start_date,
                    end_date: t.end_date,
                    created: t.created,
                    updated: t.updated,
                    user: u,
                    city: c,
                    matches,
                }
            })
            .collect::<Vec<_>>()
    })
}

fn get_matches_for_trip(
    list_of_matches: Vec<(Trip, Match, User)>,
    trip_id: i32,
    user_id: i32,
) -> Vec<MatchingTrip> {
    list_of_matches
        .into_iter()
        // Filter out your own trips
        .filter(|(_t, _m, u)| (u.id != user_id))
        // Get the matching that match this trip
        .filter(|(_t, m, _u)| (m.trip_a == trip_id || m.trip_b == trip_id))
        // Build the desired struct
        .map(|(t, _m, u)| MatchingTrip {
            user: UserPopulated {
                id: u.id,
                first_name: u.first_name,
                last_name: u.last_name,
                username: u.username,
                photo_url: u.photo_url,
                created: u.created,
                updated: u.updated,
                // Matches are friends ONLY
                is_friend: true,
                is_pending_friend: false,
                have_requested_friend: false,
                is_you: false,
            },
            start_date: t.start_date,
            end_date: t.end_date,
        })
        .collect::<Vec<_>>()
}
