use mongodb::bson::{oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub is_done: bool,
}

impl Todo {
    pub fn from_doc(doc: Document) -> Self {
        let mut todo = Todo::default();
        todo.id = doc.get_object_id("_id").unwrap().to_string();
        todo.text = doc.get_str("text").unwrap().to_owned();
        todo.is_done = doc.get_bool("is_done").unwrap();
        return todo;
    }
    pub fn into_doc(&self) -> Document {
        let mut doc = Document::new();
        if self.id != "" {
            doc.insert("_id", ObjectId::parse_str(self.id.to_owned()).unwrap());
        } else {
            doc.insert("_id", ObjectId::default());
        }
        doc.insert("text", self.text.to_owned());
        doc.insert("is_done", self.is_done);
        doc
    }
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
    pub fn into_doc(&self) -> Document {
        let mut doc = Document::new();

        if self.id.is_some() == true {
            let id: ObjectId = ObjectId::parse_str(self.id.to_owned().unwrap()).unwrap();
            doc.insert("_id", id);
        }

        if self.text.is_some() == true {
            doc.insert("text", self.text.to_owned().unwrap());
        }
        if self.is_done.is_some() == true {
            doc.insert("is_done", self.is_done.unwrap());
        }

        doc
    }
}
