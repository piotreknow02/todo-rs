use std::env;

pub struct Config {
    db_user: String,
    db_password: String,
    db_host: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            db_user: env::var("DB_USER").expect("$DB_USER not set"),
            db_password: env::var("DB_PASSWORD").expect("$DB_PASSWORD not set"),
            db_host: env::var("DB_HOST").expect("$DB_HOST not set"),
        }
    }
    
    pub fn get_connection_str(&self) -> String {
        format!("mongodb://{:?}:{:?}@{:?}/todo", self.db_user.as_str(), self.db_password.as_str(), self.db_host.as_str())
    }
}