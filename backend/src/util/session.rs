//
//  util/session.rs
//

// use actix_redis::{RedisSession, SameSite};
use actix_session::{CookieSession};
// use actix_web::dev::Transform;
use std::env;

//
// Returns a session middleware implementation based on whether a REDIS_URL has been set.
//
// TODO: Fix the types to be able to return CookieSession | RedisSession
//
pub fn get_session_middleware() -> CookieSession {
    let cookie_domain = env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN not set!");

    // if let Ok(redis_url) = env::var("REDIS_URL") {
    //     // Use redis backend if available
    //     let key = env::var("SECRET_KEY").expect("SECRET_KEY not set!");
    //     let mut redis_backend = RedisSession::new(redis_url, key.as_bytes())
    //         .cookie_name("sessionid")
    //         .cookie_secure(false)
    //         // .cookie_http_only(false) Not in main actix_redis
    //         .cookie_path("/")
    //         .cookie_same_site(SameSite::Strict);
    //
    //     if cookie_domain.contains("localhost") == false {
    //         redis_backend = redis_backend.cookie_domain(&cookie_domain);
    //     }
    //
    //     redis_backend
    // } else {
        // Otherwise use default session backend
        let mut cookie_backend = CookieSession::signed(&[0; 32])
            .name("sessionid")
            .secure(false)
            .http_only(false)
            .path("/");

        if cookie_domain.contains("localhost") == false {
            cookie_backend = cookie_backend.domain(cookie_domain);
        }

        cookie_backend
    // }
}
