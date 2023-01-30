#[macro_use]
extern crate rocket;
mod router;

use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![router::list_items])
}
