use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Todo {
    pub _id: ObjectId,
    pub text: String,
    pub is_done: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialTodo {
    pub _id: Option<ObjectId>,
    pub text: Option<String>,
    pub is_done: Option<bool>,
}

impl PartialTodo {
    pub fn into_todo(&self, fallback_todo: Option<Todo>) -> Todo {
        let fallback = match fallback_todo {
            Some(todo) => todo,
            None => Todo::default(),
        };
        Todo {
            _id: self._id.unwrap_or(fallback._id),
            text: self.text.to_owned().unwrap_or(fallback.text),
            is_done: self.is_done.unwrap_or(fallback.is_done),
        }
    }
}
