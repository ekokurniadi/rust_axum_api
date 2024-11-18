use dotenv::dotenv;
use std::env;

pub fn init_env() {
    dotenv().ok();
}

pub fn get(key: &str, default_value: String) -> String {
    env::var(key).unwrap_or(default_value)
}
