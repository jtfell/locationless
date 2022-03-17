//  src/friendships/models.rs
//
//  Implements a basic Friendship model
//
//  user_a < user_b (enforced by DB)
//  created -> friend request time
//  updated -> friend accept time
//
//  As of 28/11/2018 Diesel doesn't have support for aliases, which means that
//  you can't join on the same table twice in the same query. See:
//  https://github.com/diesel-rs/diesel/pull/1773 for progress.
//
//  I will use 2 separate queries here in the mean time...

// https://github.com/rust-lang/rust/issues/50504
#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::web;
use chrono;
use diesel;
use diesel::prelude::*;

use schema::{friendships, users};
use users::models::User;
use State;

//
// Core Friendship Model
//

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[primary_key(user_a, user_b)]
pub struct Friendship {
    pub user_a: i32,
    pub user_b: i32,
    pub user_a_accepted: bool,
    pub user_b_accepted: bool,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

//
// Friendship with User fields populated
//

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendshipPopulated {
    pub user_a: User,
    pub user_b: User,
    pub user_a_accepted: bool,
    pub user_b_accepted: bool,
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "friendships"]
pub struct NewFriendship {
    pub user_a: i32,
    pub user_b: i32,
    pub user_a_accepted: bool,
    pub user_b_accepted: bool,
}

pub fn new(
    msg: NewFriendship,
    data: web::Data<State>,
) -> Result<Friendship, diesel::result::Error> {
    use schema::friendships::dsl::*;
    let conn = data.pool.get().unwrap();

    diesel::insert_into(friendships)
        .values(&msg)
        .get_result::<Friendship>(&conn)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FriendshipUpdate {
    pub user_a: i32,
    pub user_b: i32,
    pub user_a_accepted: Option<bool>,
    pub user_b_accepted: Option<bool>,
}

pub fn update(msg: FriendshipUpdate, data: web::Data<State>) -> Result<(), diesel::result::Error> {
    use schema::friendships::dsl::*;
    let conn = data.pool.get().unwrap();

    // Do the update if there is a value for user_a_accepted
    msg.user_a_accepted.map(|user_a_acc| {
        diesel::update(friendships.filter(user_a.eq(msg.user_a).and(user_b.eq(msg.user_b))))
            .set(user_a_accepted.eq(user_a_acc))
            .execute(&conn)
    });

    // Do the update if there is a value for user_b_accepted
    msg.user_b_accepted.map(|user_b_acc| {
        let d =
            diesel::update(friendships.filter(user_a.eq(msg.user_a).and(user_b.eq(msg.user_b))))
                .set(user_b_accepted.eq(user_b_acc));

        let str = diesel::debug_query::<diesel::pg::Pg, _>(&d).to_string();
        println!("{}", str);

        diesel::update(friendships.filter(user_a.eq(msg.user_a).and(user_b.eq(msg.user_b))))
            .set(user_b_accepted.eq(user_b_acc))
            .execute(&conn)
    });
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct UserFriendshipLookup {
    pub active_user_id: i32,
    // You can look for a specific user if you like
    pub user_id: Option<i32>,
}

pub fn lookup(
    msg: UserFriendshipLookup,
    data: web::Data<State>,
) -> Result<Vec<FriendshipPopulated>, diesel::result::Error> {
    use schema::friendships::dsl::*;
    let conn = data.pool.get().unwrap();

    // Populate user A in each friendship
    let db_lookup_a_partial = friendships
        .inner_join(users::table.on(user_a.eq(users::id)))
        .order_by(user_a)
        .then_order_by(user_b);

    // Populate user B in each friendship
    let db_lookup_b_partial = friendships
        .inner_join(users::table.on(user_b.eq(users::id)))
        .order_by(user_a)
        .then_order_by(user_b);

    let db_lookup_res_a;
    let db_lookup_res_b;

    // WHERE clauses are based on whether there is a filter user supplied
    if let Some(user_filter) = msg.user_id {
        db_lookup_res_a = db_lookup_a_partial
            .filter(user_b.eq(user_filter).and(user_a.eq(msg.active_user_id)))
            .or_filter(user_b.eq(msg.active_user_id).and(user_a.eq(user_filter)))
            .get_results::<(Friendship, User)>(&conn);
        db_lookup_res_b = db_lookup_b_partial
            .filter(user_a.eq(user_filter).and(user_b.eq(msg.active_user_id)))
            .or_filter(user_a.eq(msg.active_user_id).and(user_b.eq(user_filter)))
            .get_results::<(Friendship, User)>(&conn);
    } else {
        db_lookup_res_a = db_lookup_a_partial
            .filter(
                user_b
                    .eq(msg.active_user_id)
                    .or(user_a.eq(msg.active_user_id)),
            )
            .get_results::<(Friendship, User)>(&conn);
        db_lookup_res_b = db_lookup_b_partial
            .filter(
                user_b
                    .eq(msg.active_user_id)
                    .or(user_a.eq(msg.active_user_id)),
            )
            .get_results::<(Friendship, User)>(&conn);
    }

    // Combine the 2 results into a single populated result. As each query above
    // has the same WHERE and ORDER BY clauses, the array indexes will be the same.
    match db_lookup_res_a {
        Err(e_a) => Err(e_a),
        Ok(f_list_a) => {
            match db_lookup_res_b {
                Err(e_b) => Err(e_b),
                Ok(f_list_b) => {
                    let list_a = f_list_a.into_iter();
                    let list_b = f_list_b.into_iter();
                    let populated_list = list_a
                        .zip(list_b)
                        .map(|((f_a, u_a), (f_b, u_b))| {
                            // Ensure that the result lists match up
                            assert!(f_a.user_a == f_b.user_a);
                            assert!(f_a.user_b == f_b.user_b);

                            FriendshipPopulated {
                                user_a: u_a,
                                user_b: u_b,
                                user_a_accepted: f_a.user_a_accepted,
                                user_b_accepted: f_a.user_b_accepted,
                            }
                        })
                        .collect::<Vec<_>>();

                    // If a specific user is requested, we shouldn't be returning more than
                    // 1 result from this method
                    if let Some(_) = msg.user_id {
                        assert!(populated_list.len() == 1 || populated_list.len() == 0);
                    }

                    Ok(populated_list)
                }
            }
        }
    }
}
