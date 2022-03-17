//  users/mod.rs
//
//  URL dispatcher for user account related API endpoints.

use actix_web::{web, Scope};

pub mod handlers;
pub mod middleware;
pub mod models;

pub fn scope_factory() -> Scope {
    // Auth-based user actions
    web::scope("/u")
        .route("/auth", web::get().to_async(handlers::Auth::get))
        .route("/logout", web::post().to_async(handlers::logout))
        // Info about your own user
        .route("/", web::get().to_async(handlers::Lookup::get_self))
        // Lookup a list of your friends/friend requests
        .route(
            "/friends",
            web::get().to_async(handlers::Friendships::get_all),
        )
        // Actions specific to other users
        .route("/friends", web::get().to_async(handlers::Search::search))
        .route("/{id}", web::get().to_async(handlers::Lookup::get))
        .route(
            "/{username}/preview",
            web::get().to_async(handlers::PreviewLookup::get),
        )
        .route("/{id}/trips", web::get().to_async(handlers::GetTrips::get))
        .route(
            "/{id}/friends",
            web::put().to_async(handlers::FriendResponse::response),
        )
        .route(
            "/{id}/friends",
            web::post().to_async(handlers::FriendRequest::request),
        )
}
