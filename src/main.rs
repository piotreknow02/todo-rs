#[macro_use]
extern crate rocket;
mod router;
mod database;
mod models;
mod config;

use database::DbRepo;
use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db = DbRepo::connect();
    rocket::build()
        .manage(db)
        .mount("/", routes![router::get_items, router::add_item, router::mark_as_done, router::delete_item])
}
