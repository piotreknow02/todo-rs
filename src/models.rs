use rocket::serde::{Serialize};

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Item<'a> {
    pub id: u16,
    pub title: &'a str,
    pub description: &'a str,
    pub is_done: bool,
}