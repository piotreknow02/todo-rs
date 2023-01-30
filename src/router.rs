#[path ="models.rs"]
mod models;

use models::Item;
use rocket::http::Status;
use rocket::serde::json::{json, Value};

#[get("/")]
pub fn list_items() -> Result<Value, Status> {
    Ok(
        json!(vec![Item{id: 1, title: "Test", description: "test", is_done: false}])
    )
}