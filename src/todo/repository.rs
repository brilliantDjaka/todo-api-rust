use ::mongodb::{error::Error, Client, Collection};
use futures::stream::TryStreamExt;
use rocket::futures;

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
        self.get_collection().insert_one(&todo, None).await?;
        Ok(todo)
    }
    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let todos = self.get_collection().find(None, None).await?;
        let todos: Vec<Todo> = todos.try_collect().await?;

        Ok(todos)
    }
}

pub fn new(db: Client) -> TodoRepository {
    TodoRepository { db }
}
