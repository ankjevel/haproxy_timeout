use rocket::config::Environment;
use std::boxed::Box;
use std::env;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Config {
    pub port: u16,
    pub timeout: usize,
    pub host: &'static str,
    pub env: Environment,
}

impl Config {
    pub fn new() -> Self {
        let port = match env::var("PORT") {
            Ok(port) => port.parse().unwrap(),
            Err(_) => 3000,
        };

        let timeout = match env::var("TIMEOUT") {
            Ok(timeout) => timeout.parse().unwrap(),
            Err(_) => 1000,
        };

        let host: &'static str = match env::var("HOST") {
            Ok(host) => Box::leak(host.into_boxed_str()),
            Err(_) => "0.0.0.0",
        };

        let env = match env::var("ENV") {
            Ok(env) => {
                let lower = env.to_lowercase();
                if lower.starts_with("dev") {
                    Environment::Development
                } else if lower.starts_with("prod") {
                    Environment::Production
                } else if lower.starts_with("stage") {
                    Environment::Staging
                } else {
                    Environment::Development
                }
            }
            Err(_) => Environment::Development,
        };

        Config {
            port,
            timeout,
            host,
            env,
        }
    }
}
