use super::{controller::AddTodoDto, entity::Todo, repository::TodoRepository};
use mongodb::error::Error;

pub struct TodoService {
    todo_repo: TodoRepository,
}

impl TodoService {
    pub async fn add_todo(&self, dto: AddTodoDto) -> Result<Todo, Error> {
        let todo = Todo {
            id: String::from("dummy"),
            text: dto.text,
            is_done: true,
        };

        self.todo_repo.insert_new_todo(todo).await
    }
}

pub fn new(todo_repo: TodoRepository) -> TodoService {
    TodoService { todo_repo }
}
