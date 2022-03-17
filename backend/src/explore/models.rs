//  src/explore/models.rs
//
//  Implements DB operations for suggesting trip options

// https://github.com/rust-lang/rust/issues/50504
#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::web;
use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use cities::models::City;
use friendships::models::Friendship;
use schema::{cities, friendships, users};
use trips::models::Trip;
use users::models::User;
use State;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Suggestion {
    pub city: City,
    pub users: Vec<User>,
}

#[derive(Debug, Clone)]
pub enum ContinentCode {
    EU,
    NA,
    SA,
    AF,
    AS,
    OC,
}

impl Serialize for ContinentCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            ContinentCode::EU => "EU",
            ContinentCode::NA => "NA",
            ContinentCode::SA => "SA",
            ContinentCode::AF => "AF",
            ContinentCode::AS => "AS",
            ContinentCode::OC => "OC",
        })
    }
}

// TODO: You can do better than this (maybe use https://docs.rs/serde_plain/0.3.0/serde_plain/)
fn continent_to_string(c: ContinentCode) -> String {
    match c {
        ContinentCode::EU => "EU".to_string(),
        ContinentCode::NA => "NA".to_string(),
        ContinentCode::SA => "SA".to_string(),
        ContinentCode::AF => "AF".to_string(),
        ContinentCode::AS => "AS".to_string(),
        ContinentCode::OC => "OC".to_string(),
    }
}

impl<'de> Deserialize<'de> for ContinentCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "EU" => Ok(ContinentCode::EU),
            "NA" => Ok(ContinentCode::NA),
            "SA" => Ok(ContinentCode::SA),
            "AF" => Ok(ContinentCode::AF),
            "AS" => Ok(ContinentCode::AS),
            "OC" => Ok(ContinentCode::OC),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid continent code: {}",
                s
            ))),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ExploreLookup {
    pub active_user_id: i32,
    pub continent: Option<ContinentCode>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

pub fn lookup(
    msg: ExploreLookup,
    data: web::Data<State>,
) -> Result<Vec<Suggestion>, diesel::result::Error> {
    use schema::trips::dsl::*;
    let conn = data.pool.get().unwrap();

    //
    // Gets all cities with at least 1 friend having a trip there
    // within the specified constraints (continent, dates)
    //
    let mut query = trips
        .inner_join(cities::table.on(cities::id.eq(city)))
        .inner_join(users::table.on(users::id.eq(user)))
        // Both sides of the friendship need to align with you + your friend
        .inner_join(
            friendships::table.on((friendships::user_a
                .eq(user)
                .and(friendships::user_b.eq(msg.active_user_id)))
            .or(friendships::user_b
                .eq(user)
                .and(friendships::user_a.eq(msg.active_user_id)))),
        )
        // Trips cant be with yourself
        .filter(user.ne(msg.active_user_id))
        // Only show results for friends
        .filter(friendships::user_a_accepted.eq(true))
        .filter(friendships::user_b_accepted.eq(true))
        // Needed to conditionally modify a query
        .into_boxed();

    // Filter by continent
    if let Some(c) = msg.continent {
        query = query.filter(cities::continent_code.eq(continent_to_string(c)));
    };

    // Filter by dates
    if let Some(e) = msg.end_date {
        query = query.filter(start_date.le(e));
    };
    if let Some(s) = msg.start_date {
        query = query.filter(end_date.gt(s));
    };

    query
        .order_by(city)
        .get_results::<(Trip, City, User, Friendship)>(&conn)
        .map(move |explore_res| {
            // As diesel lacks a way to do group_bys:
            //
            //   https://github.com/diesel-rs/diesel/issues/210
            //
            // We do all our grouping, ordering and nesting manually.

            // Get the list of cities
            let cities_list = explore_res
                .clone()
                .into_iter()
                .map(move |(_t, c, _u, _f)| c)
                .collect::<Vec<_>>();

            cities_list
                .into_iter()
                .map(move |city_inner| {
                    // Populate it with matches
                    let matches = explore_res
                        .clone()
                        .into_iter()
                        .filter(|(_t, c, _u, _f)| c.id == city_inner.id)
                        .map(move |(_t, _c, u, _f)| u)
                        .collect::<Vec<_>>();

                    // TODO: Calculate trip_days metric and it use it to sort results

                    Suggestion {
                        city: city_inner,
                        users: matches,
                    }
                })
                .collect::<Vec<_>>()
        })
}
