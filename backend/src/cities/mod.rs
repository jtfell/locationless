//  trips/mod.rs
//
//  URL dispatcher for trip related API endpoints.

use actix_web::{web, Scope};

pub mod handlers;
pub mod models;

pub fn scope_factory() -> Scope {
    web::scope("/cities")
        .route("/", web::get().to_async(handlers::LookupAll::get_all))
        .route("/{id}", web::get().to_async(handlers::Lookup::get))
}
