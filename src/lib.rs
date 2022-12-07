pub mod err;
pub mod mongodb;
pub mod todo;
use todo::service::TodoService;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService,
}
