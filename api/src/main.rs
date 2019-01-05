#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod config;
mod middleware;
mod models;
mod routes;

use rocket::config::Config;
use std::sync::atomic::AtomicUsize;

lazy_static! {
    static ref CONFIG: config::Config = { config::Config::new() };
}

fn main() {
    let config = &CONFIG;

    let rocket_config = Config::build(config.env)
        .address(config.host)
        .port(config.port)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config)
        .manage(middleware::Timeout {
            current: AtomicUsize::new(config.timeout),
        })
        .mount("/", routes![routes::index::get_index])
        .mount("/health", routes![routes::health::get_health])
        .mount("/timeout", routes![routes::timeout::post_timeout])
        .launch();
}
