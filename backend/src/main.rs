//
//  main.rs
//

//
// Needed until the fix for this is released:
// https://github.com/diesel-rs/diesel/issues/1785
//
#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix_web;
extern crate actix_session;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate num_cpus;
extern crate sentry;
extern crate sentry_actix;
extern crate serde;
extern crate telegram_bot_fork;
extern crate telegram_login;
extern crate uuid;
extern crate validator;
#[macro_use]
extern crate log;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
// use sentry_actix::SentryMiddleware;
use std::env;

pub mod cities;
pub mod explore;
pub mod friendships;
pub mod misc;
pub mod schema;
pub mod trips;
pub mod users;
pub mod util;

pub struct State {
    pub pool: util::database::PgPool,
    pub telegram: util::telegram::Telegram
}

fn main() {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug,info,warn");
    env_logger::init();

    // let _sentry;
    // if let Ok(dsn) = env::var("SENTRY_DSN") {
    //     _sentry = sentry::init(dsn);
    //     sentry::integrations::panic::register_panic_handler();
    // }

    let address = env::var("BIND_TO").expect("BIND_TO not set!");
    // let sys = System::new("locationless-backend");

    let pool = util::database::pool();
    let telegram = util::telegram::Telegram::new();

    HttpServer::new(move || {
        App::new().data(State {
            pool: pool.clone(),
            telegram: telegram.clone()
        })

        // Disabled pending resolution to this issue:
        // https://github.com/getsentry/sentry-rust/issues/143
        //
        // .wrap(SentryMiddleware::new())

        .wrap(middleware::Logger::default())
        .wrap(util::session::get_session_middleware())

        .service(misc::scope_factory())
        .service(users::scope_factory())
        .service(cities::scope_factory())
        .service(trips::scope_factory())
        .service(explore::scope_factory())
    })
    // .backlog(8192)
    .workers(4)
    .bind(&address)
    .unwrap()
    .run();
}
