//  src/cities/models.rs
//
//  Implements a basic City model

// https://github.com/rust-lang/rust/issues/50504
#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::web;
use diesel::prelude::*;

use schema::cities;
use State;

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[table_name = "cities"]
pub struct City {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country_code: String,
    pub continent_code: String,
    pub population: i32,
}

//
// Pair of In/Outs for the /cities/{id} handler
//
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CityLookup {
    pub id: i32,
}

pub fn lookup(msg: CityLookup, data: web::Data<State>) -> Result<City, diesel::result::Error> {
    use schema::cities::dsl::*;
    let conn = data.pool.get().unwrap();

    cities.filter(id.eq(msg.id)).get_result::<City>(&conn)
}

//
// Pair of In/Outs for the /cities handler
//
#[derive(Serialize, Deserialize, Debug)]
pub struct CityLookupAll {}

pub fn lookup_all(
    msg: CityLookupAll,
    data: web::Data<State>,
) -> Result<Vec<City>, diesel::result::Error> {
    use schema::cities::dsl::*;
    let conn = data.pool.get().unwrap();

    cities.get_results::<City>(&conn)
}
