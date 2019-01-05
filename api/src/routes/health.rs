use super::super::middleware::Timeout;
use super::super::models::Health;

use rocket::State;
use rocket_contrib::json::Json;
use std::sync::atomic::Ordering::Relaxed;
use std::{thread, time};

#[get("/")]
pub fn get_health(timeout: State<Timeout>) -> Json<Health> {
    let current = timeout.current.load(Relaxed);
    let current_in_millis = time::Duration::from_millis(current as u64);
    let started = time::Instant::now();

    thread::sleep(current_in_millis);

    Json(Health::new(time::Instant::now() - started))
}
