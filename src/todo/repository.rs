use ::mongodb::{error::Error, Client};

use crate::mongodb;

use super::entity::Todo;

const COLLECTION_NAME: &str = "todos";

pub struct TodoRepository {
    db: Client,
}

impl TodoRepository {
    pub async fn insert_new_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let collection = self
            .db
            .database(mongodb::DB_NAME)
            .collection::<Todo>(COLLECTION_NAME);
        let insert_res = collection.insert_one(&todo, None).await?;
        Ok(Todo {
            id: insert_res.inserted_id.as_object_id().unwrap().to_string(),
            text: todo.text,
            is_done: todo.is_done,
        })
    }
}

pub fn new(db: Client) -> TodoRepository {
    TodoRepository { db }
}
