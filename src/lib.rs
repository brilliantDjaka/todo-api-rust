pub mod err;
pub mod mongodb;
pub mod todo;
use std::sync::Arc;

use err::Error;
use todo::service::TodoService;
use validator::Validate;

pub struct AppState {
    pub todo_service: Arc<TodoService>,
}

trait ValidateRequest: Validate {
    fn validate_req(&self) -> Option<Error> {
        match self.validate() {
            Ok(_) => return None,
            Err(err) => Some(Error::BadRequestError(Some(err.to_string()))),
        }
    }
}
