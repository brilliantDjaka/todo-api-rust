use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub _id: ObjectId,
    pub text: String,
    pub is_done: bool,
}
