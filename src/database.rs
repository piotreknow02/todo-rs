use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
#[path = "config.rs"]
use config::*;
#[path = "model.rs"]
use model::Item;

const CONNECTION_STR: &'static str = from_env().get_connection_str();

struct DbConnector {
    client: Client
}

impl DbConnector {
    pub async fn connect() -> Self {
        let mut client_options = ClientOptions.parse(CONNECTION_STR).await?;
        client_options.app_name = Some("todo-rs".to_string());
    
        let client = Client::with_options(client_options)?;
        Self{client: client}
    }

    pub async fn get_items() -> Result<Vec<Item>, Error> {
        todo!("Get items from db")
    }

    pub async fn insert_item(item: &Item) -> Option<Error> {
        todo!("Insert {:?} item to db", item)
    }

    pub async fn mark_as_done(id: u16) -> Option<Error> {
        todo!("Mark todo {:?} as done", id)
    }

    pub async fn remove_item(id: u16) -> Option<Error> {
        todo!("Remove item with id {:?}", id)
    }
}
