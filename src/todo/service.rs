use std::sync::Arc;

use super::{
    controller::{AddTodoDto, UpdateTodoDto},
    entity::Todo,
    repository::TodoRepository,
};
use crate::err::Error;
use mongodb::bson::oid::ObjectId;

pub struct TodoService {
    todo_repo: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub async fn add_todo(&self, dto: AddTodoDto) -> Result<Todo, Error> {
        let todo = Todo {
            _id: ObjectId::default(),
            text: dto.text,
            is_done: false,
        };

        self.todo_repo.insert_new_todo(todo).await
    }
    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        self.todo_repo.get_all().await
    }
    pub async fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Error> {
        self.todo_repo.get_by_id(id).await
    }

    pub async fn update_by_id(&self, id: &str, dto: UpdateTodoDto) -> Option<Error> {
        self.todo_repo
            .update_by_id(id, dto.into_partial_todo())
            .await
    }
    pub async fn delete_by_id(&self, id: &str) -> Option<Error> {
        self.todo_repo.delete_by_id(id).await
    }
}

pub fn new(todo_repo: Arc<dyn TodoRepository>) -> TodoService {
    TodoService { todo_repo }
}
