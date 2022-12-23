use ::mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;

use super::entity::{PartialTodo, Todo};
use crate::err::Error;

const COLLECTION_NAME: &str = "todos";

#[derive(Clone)]
pub struct TodoRepository {
    db: Client,
}

impl TodoRepository {
    fn get_collection(&self) -> Collection<Todo> {
        self.db
            .default_database()
            .unwrap()
            .collection::<Todo>(COLLECTION_NAME)
    }
    pub async fn insert_new_todo(&self, todo: Todo) -> Result<Todo, Error> {
        match self.get_collection().insert_one(&todo, None).await {
            Ok(_) => Ok(todo),
            Err(_) => Err(Error::InternalServerError(None)),
        }
    }
    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let todos = match self
            .get_collection()
            .find(None, FindOptions::builder().limit(20).build())
            .await
        {
            Ok(todos) => todos,
            Err(_) => return Err(Error::InternalServerError(None)),
        };
        let todos: Result<Vec<Todo>, mongodb::error::Error> = todos.try_collect().await;

        match todos {
            Ok(todos) => Ok(todos),
            Err(_) => Err(Error::InternalServerError(None)),
        }
    }
    pub async fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Error> {
        let _id = ObjectId::parse_str(id).unwrap_or_default();
        match self
            .get_collection()
            .find_one(
                doc! {
                    "_id": _id,
                },
                None,
            )
            .await
        {
            Ok(todo) => Ok(todo),
            Err(_) => Err(Error::NotFoundError(None)),
        }
    }

    pub async fn update_by_id(&self, id: &str, todo: PartialTodo) -> Option<Error> {
        let _id = ObjectId::parse_str(id).unwrap_or_default();

        let update_doc = todo.into_doc();

        match self
            .get_collection()
            .update_one(
                doc! {"_id": _id},
                doc! {
                    "$set": update_doc
                },
                None,
            )
            .await
        {
            Err(_) => Some(Error::InternalServerError(None)),
            Ok(_) => None,
        }
    }
    pub async fn delete_by_id(&self, id: &str) -> Option<Error> {
        let _id = ObjectId::parse_str(id).unwrap_or_default();
        let result = self
            .get_collection()
            .delete_one(
                doc! {
                    "_id": _id
                },
                None,
            )
            .await;
        match result {
            Ok(_) => None,
            Err(_) => Some(Error::InternalServerError(None)),
        }
    }
}

pub fn new(db: Client) -> TodoRepository {
    TodoRepository { db }
}
