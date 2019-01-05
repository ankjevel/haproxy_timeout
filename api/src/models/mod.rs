use super::CONFIG;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTimeout {
    pub timeout: usize,
}

#[derive(Serialize, Deserialize)]
pub struct HealthComponent {
    pub time: u128,
    pub port: u16,
    pub healthy: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub oracle: HealthComponent,
}

impl Health {
    pub fn new(duration: Duration) -> Self {
        let config = &CONFIG;
        let time = duration.as_millis();

        Health {
            oracle: HealthComponent {
                time,
                port: config.port,
                healthy: time < 30000,
            },
        }
    }
}
