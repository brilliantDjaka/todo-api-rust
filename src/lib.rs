pub mod mongodb;
pub mod todo;
pub mod err;
use todo::service::TodoService;

pub struct AppState {
    pub todo_service: TodoService,
}
