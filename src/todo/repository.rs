use ::mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use futures::stream::TryStreamExt;
use mongodb::bson::to_document;
use rocket::{futures, http::Status};

use crate::mongodb::DB_NAME;

use super::entity::Todo;

const COLLECTION_NAME: &str = "todos";

pub struct TodoRepository {
    db: Client,
}

impl TodoRepository {
    fn get_collection(&self) -> Collection<Todo> {
        self.db
            .database(DB_NAME)
            .collection::<Todo>(COLLECTION_NAME)
    }
    pub async fn insert_new_todo(&self, todo: Todo) -> Result<Todo, Status> {
        match self.get_collection().insert_one(&todo, None).await {
            Ok(_) => Ok(todo),
            Err(_) => Err(Status::InternalServerError),
        }
    }
    pub async fn get_all(&self) -> Result<Vec<Todo>, Status> {
        let todos = match self.get_collection().find(None, None).await {
            Ok(todos) => todos,
            Err(_) => return Err(Status::InternalServerError),
        };
        let todos: Result<Vec<Todo>, mongodb::error::Error> = todos.try_collect().await;

        match todos {
            Ok(todos) => Ok(todos),
            Err(_) => Err(Status::InternalServerError),
        }
    }
    pub async fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Status> {
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
            Err(_) => Err(Status::NotFound),
        }
    }

    //TODO: Implement Partial Update
    pub async fn update_by_id(&self, id: &str, todo: Todo) -> Result<Todo, Status> {
        let _id = ObjectId::parse_str(id).unwrap_or_default();
        let todo = Todo { _id, ..todo };
        let update_doc = match to_document(&todo) {
            Err(_) => return Err(Status::InternalServerError),
            Ok(data) => data,
        };
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
            Err(_) => Err(Status::InternalServerError),
            Ok(_) => Ok(todo),
        }
    }
    pub async fn delete_by_id(&self, id: &str) -> Option<Status> {
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
            Err(_) => Some(Status::InternalServerError),
        }
    }
}

pub fn new(db: Client) -> TodoRepository {
    TodoRepository { db }
}
