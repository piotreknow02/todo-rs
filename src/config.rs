use std::env;

pub struct Config {
    port: u16,
    db_user: &str,
    db_password: &str,
    db_host: &str,

}

pub fn from_env(&mut self) -> Config {
    let mut conf: Config = Config::new();
    self.port = env::var("PORT").expect("$PORT not set");
    self.db_user = env::var("DB_USER").expect("$DB_USER not set");
    self.db_password = env::var("DB_PASSWORD").expect("$DB_PASSWORD not set");
    self.db_host = env::var("DB_HOST").expect("$DB_HOST not set");
    conf
}

impl Config {
    pub fn get_connection_str(&self) -> &str {
        format!("mongodb+srv://{:?}:{:?}@{:?}/todo", self.db_user, self.password, self.db_host)
    }
}