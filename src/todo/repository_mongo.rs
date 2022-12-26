use ::mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client, Collection,
};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;

use super::entity::{PartialTodo, Todo};
use super::repository::TodoRepository;
use crate::{err::Error, mongodb::EntityDoc};

const COLLECTION_NAME: &str = "todos";

pub struct TodoRepositoryMongo {
    db: Client,
}

impl TodoRepositoryMongo {
    fn get_collection(&self) -> Collection<Document> {
        self.db
            .default_database()
            .unwrap()
            .collection::<Document>(COLLECTION_NAME)
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryMongo {
    async fn insert_new_todo(&self, todo: Todo) -> Result<Todo, Error> {
        match self
            .get_collection()
            .insert_one(todo.into_doc(), None)
            .await
        {
            Ok(_) => Ok(todo),
            Err(_) => Err(Error::InternalServerError(None)),
        }
    }
    async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let todos = match self
            .get_collection()
            .find(None, FindOptions::builder().limit(20).build())
            .await
        {
            Ok(todos) => todos,
            Err(_) => return Err(Error::InternalServerError(None)),
        };
        let todos: Result<Vec<Document>, mongodb::error::Error> = todos.try_collect().await;

        let todos = match todos {
            Ok(todos) => todos,
            Err(_) => return Err(Error::InternalServerError(None)),
        };
        let mut mapped_todo: Vec<Todo> = vec![];
        for todo in todos {
            mapped_todo.push(Todo::from_doc(todo));
        }
        return Ok(mapped_todo);
    }
    async fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Error> {
        let _id = ObjectId::parse_str(id).unwrap_or_default();
        let todo = self
            .get_collection()
            .find_one(
                doc! {
                    "_id": _id,
                },
                None,
            )
            .await;
        let todo = match todo {
            Err(_) => {
                return Err(Error::InternalServerError(Some(
                    "Cant find data".to_owned(),
                )))
            }
            Ok(todo) => todo,
        };

        match todo {
            None => Ok(None),
            Some(todo) => Ok(Some(Todo::from_doc(todo))),
        }
    }

    async fn update_by_id(&self, id: &str, todo: PartialTodo) -> Option<Error> {
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
    async fn delete_by_id(&self, id: &str) -> Option<Error> {
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

pub fn new(db: Client) -> TodoRepositoryMongo {
    TodoRepositoryMongo { db }
}

impl EntityDoc for Todo {
    fn from_doc(doc: Document) -> Self {
        let mut todo = Todo::default();
        todo.id = doc.get_object_id("_id").unwrap().to_string();
        todo.text = doc.get_str("text").unwrap().to_owned();
        todo.is_done = doc.get_bool("is_done").unwrap();
        return todo;
    }
    fn into_doc(&self) -> Document {
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

impl EntityDoc for PartialTodo {
    fn into_doc(&self) -> Document {
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

    fn from_doc(_: Document) -> Self {
        todo!("Not Implemented")
    }
}
