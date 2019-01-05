use super::super::middleware::Timeout;
use super::super::models::{Message, UpdateTimeout};

use rocket::State;
use rocket_contrib::json::Json;
use std::sync::atomic::Ordering;

#[post("/", format = "json", data = "<update_timeout>")]
pub fn post_timeout(
    update_timeout: Option<Json<UpdateTimeout>>,
    timeout: State<Timeout>,
) -> Json<Message> {
    match update_timeout {
        Some(unwrapped) => {
            timeout.current.swap(unwrapped.timeout, Ordering::Relaxed);

            Json(Message {
                message: "OK".to_string(),
            })
        }
        None => Json(Message {
            message: "FAIL".to_string(),
        }),
    }
}
