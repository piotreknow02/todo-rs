use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, 
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use super::config::*;
use super::models::Item;

pub struct DbRepo {
    collection: Collection<Item>,
}

impl DbRepo {
    pub fn connect() -> Self {
        let conf = Config::from_env();
        let connection_str = conf.get_connection_str();
        let client = Client::with_uri_str(connection_str).unwrap();
        let db = client.database("todo");
        let col: Collection<Item> = db.collection("Item");
        Self{collection: col}
    }
        
    pub fn get_items(&self) -> Result<Vec<Item>, Error> {
        let items = self
            .collection
            .find(None, None)
            .ok()
            .expect("Error getting items");
        Ok(items.map(|i| i.unwrap()).collect())
    }

    pub fn insert_item(&self, item: &Item) -> Result<InsertOneResult, Error> {
        let inserted_doc = self
            .collection
            .insert_one(item, None)
            .ok()
            .expect("Error inserting item");
        Ok(inserted_doc)
    }

    pub fn mark_as_done(&self, id: &String) -> Result<UpdateResult, Error> {
        let oid = ObjectId::parse_str(&id).unwrap();
        let filter = doc! {"_id": oid};
        let new_doc = doc! {
            "$set": {
                "is_done": true,
            }
        };
        let updated_doc = self
            .collection
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating item");
        Ok(updated_doc)
    }

    pub fn remove_item(&self, id: &String) -> Result<DeleteResult, Error> {
        let oid = ObjectId::parse_str(&id).unwrap();
        let filter = doc! {"_id": oid};
        let deleted_doc = self
            .collection
            .delete_one(filter, None)
            .ok()
            .expect("Error removing item");
        Ok(deleted_doc)
    }
}
