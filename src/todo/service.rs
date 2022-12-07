use super::{controller::AddTodoDto, entity::Todo, repository::TodoRepository};
use mongodb::{bson::oid::ObjectId, error::Error};

pub struct TodoService {
    todo_repo: TodoRepository,
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
}

pub fn new(todo_repo: TodoRepository) -> TodoService {
    TodoService { todo_repo }
}
