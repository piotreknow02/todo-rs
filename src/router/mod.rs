use rocket::{
    http::Status,
    serde::json::{json, Json, Value},
    State,
};
use super::database::DbRepo;
use super::models::Item;

#[get("/")]
pub fn get_items(db: &State<DbRepo>) -> Result<Value, Status> {
    match db.get_items() {
        Ok(res) => {
            Ok(json!(res))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

#[post("/", data = "<new_item>")]
pub fn add_item(db: &State<DbRepo>, new_item: Json<Item>) -> Result<Value, Status> {
    let to_insert = Item {
        id: None,
        title: new_item.title.to_owned(),
        description: new_item.description.to_owned(),
        is_done: new_item.is_done,
    };
    match db.insert_item(&to_insert) {
        Ok(res) => {
            Ok(json!(res))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

#[delete("/<id>")]
pub fn delete_item(db: &State<DbRepo>, id: String) -> Result<Value, Status> {
    match db.remove_item(&id) {
        Ok(res) => {
            Ok(json!(res))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>/done")]
pub fn mark_as_done(db: &State<DbRepo>, id: String) -> Result<Value, Status> {
    match db.mark_as_done(&id) {
        Ok(res) => {
            Ok(json!(res))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}