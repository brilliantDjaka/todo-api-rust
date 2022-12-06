use ::mongodb::{error::Error, Client, Collection};

use crate::mongodb;

use super::entity::Todo;

const COLLECTION_NAME: &str = "todos";

pub struct TodoRepository {
    db: Client,
}

impl TodoRepository {
    fn get_collection(&self) -> Collection<Todo> {
        self.db
            .database(mongodb::DB_NAME)
            .collection::<Todo>(COLLECTION_NAME)
    }
    pub async fn insert_new_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let insert_res = self.get_collection().insert_one(&todo, None).await?;
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
