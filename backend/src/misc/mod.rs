//  misc/mod.rs
//
//  URL dispatcher for misc endpoints.

use actix_web::{web, Scope};

pub mod handlers;

pub fn scope_factory() -> Scope {
    web::scope("/x").route("/status", web::get().to_async(handlers::Status::status))
}
