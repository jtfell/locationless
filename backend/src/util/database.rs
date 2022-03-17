//  database.rs
//
//  Handles setting up database routines, state, and such
//  to work within actix-web.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/16/2018

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_types::{Integer, Nullable, Text};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Error creating Postgres connection pool!")
}

pub struct Database(pub Pool<ConnectionManager<PgConnection>>);

//
// Declare any postgres functions for use in the query builder
//
// TODO: Is there a better way to express the number of args? Eg. least should take 2 or more args
//
sql_function!(fn levenshtein(x: Nullable<Text>, y: Nullable<Text>) -> Nullable<Integer>);
sql_function!(fn lower(x: Nullable<Text>) -> Nullable<Text>);
sql_function!(fn concat(x: Nullable<Text>, y: Nullable<Text>, z: Nullable<Text>) -> Nullable<Text>);
sql_function!(fn least(a: Nullable<Integer>, b: Nullable<Integer>, c: Nullable<Integer>, d: Nullable<Integer>) -> Nullable<Integer>);
