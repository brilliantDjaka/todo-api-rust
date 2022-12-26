use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub is_done: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialTodo {
    pub id: Option<String>,
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
            id: self.id.to_owned().unwrap_or(fallback.id),
            text: self.text.to_owned().unwrap_or(fallback.text),
            is_done: self.is_done.unwrap_or(fallback.is_done),
        }
    }
}
