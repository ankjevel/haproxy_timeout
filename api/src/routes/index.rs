use super::super::middleware::Timeout;
use super::super::models::Message;
use super::super::CONFIG;

use rocket::State;
use rocket_contrib::json::Json;
use std::sync::atomic::Ordering::Relaxed;

#[get("/")]
pub fn get_index(timeout: State<Timeout>) -> Json<Message> {
    let config = &CONFIG;

    let current = timeout.current.load(Relaxed);

    Json(Message {
        message: format!("timeout: {}, port: {}", current, config.port),
    })
}
