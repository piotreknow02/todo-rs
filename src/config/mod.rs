use std::env;

use mongodb::options::{ServerAddress, ClientOptions, Credential};

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
    
    pub fn get_db_client_options(&self) -> ClientOptions {
        let credentials = Credential::builder()
            .username(self.db_user.to_owned())
            .password(self.db_password.to_owned())
            .build();
        let client_options = ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp { host: self.db_host.to_owned(), port: Some(27017) }])
            .credential(credentials)
            .build();
        return client_options;
    }
}