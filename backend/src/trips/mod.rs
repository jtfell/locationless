//  trips/mod.rs
//
//  URL dispatcher for trip related API endpoints.

use actix_web::{web, Scope};

pub mod handlers;
pub mod models;

pub fn scope_factory() -> Scope {
    web::scope("/trips")
        .route("/", web::post().to_async(handlers::Create::create))
        .route("/{id}", web::get().to_async(handlers::Lookup::get))
        .route("/{id}", web::put().to_async(handlers::Update::update))
        .route("/{id}", web::delete().to_async(handlers::Delete::delete))
}
