use rocket::serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}