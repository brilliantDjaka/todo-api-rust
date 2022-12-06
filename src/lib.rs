pub mod mongodb;
pub mod todo;

use todo::service::TodoService;

pub struct AppState {
    pub todo_service: TodoService,
}
