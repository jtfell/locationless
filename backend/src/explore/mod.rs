//  explore/mod.rs
//
//  URL dispatcher for explore suggestion related API endpoints.

use actix_web::{web, Scope};

pub mod handlers;
pub mod models;

pub fn scope_factory() -> Scope {
    web::scope("/explore").route("/", web::get().to_async(handlers::Lookup::get))
}

// TODO: Make this work!
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use actix::sync::SyncArbiter;
//     use actix::Supervisor;
//     use actix_web::{http, test};
//     use std::env;
//
//     use util;
//
//     fn get_build_state() -> State {
//         // Make sure we're using the test DB instance
//         env::set_var("DATABASE_URL", "postgresql://localhost/locationless_test");
//         env::set_var("BOT_TOKEN", "my_bot_token");
//         env::set_var("COOKIE_DOMAIN", "locationless.local");
//
//         let pool = util::database::pool();
//         let db_addr = SyncArbiter::start(1, move || util::database::Database(pool.clone()));
//         let tg_addr = Supervisor::start(move |_| util::telegram::Telegram::new());
//         State {
//             db: db_addr.clone(),
//             telegram: tg_addr.clone(),
//         }
//     }
//
//     #[test]
//     #[ignore] // Only run locally for now
//     fn integration_test_explore() {
//         // Setup the server
//         let mut srv = test::TestServer::build_with_state(|| get_build_state())
//             // Register middlewares and handlers
//             .start(move |app| {
//                 app.middleware(util::session::get_session_middleware())
//                     .resource("/explore/", |r| {
//                         r.method(http::Method::GET).with(handlers::Lookup::get)
//                     });
//             });
//
//         println!("Hello?");
//
//         // Execute the request
//         let req = srv.client(http::Method::GET, "/explore/").finish().unwrap();
//         let res = srv.execute(req.send()).unwrap();
//
//         println!("Res - {:?}", res);
//
//         // Check the result
//         assert_eq!(http::StatusCode::UNAUTHORIZED, res.status());
//     }
// }
