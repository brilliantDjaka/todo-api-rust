use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub is_done: bool,
}
