use ::mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    error::Error,
    Client, Collection,
};
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
    pub async fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Error> {
        let _id = ObjectId::parse_str(id).unwrap_or_default();
        let todo = self
            .get_collection()
            .find_one(
                doc! {
                    "_id": _id,
                },
                None,
            )
            .await?;

        Ok(todo)
    }

    //TODO: Implement Partial Update
    pub async fn update_by_id(&self, id: &str, todo: Todo) -> Result<Todo, Error> {
        let _id = ObjectId::parse_str(id).unwrap_or_default();
        let todo = Todo { _id, ..todo };
        let update_doc = to_document(&todo)?;
        self.get_collection()
            .update_one(
                doc! {"_id": _id},
                doc! {
                    "$set": update_doc
                },
                None,
            )
            .await?;
        Ok(todo)
    }
}

pub fn new(db: Client) -> TodoRepository {
    TodoRepository { db }
}
