use async_trait::async_trait;

use super::entity::{PartialTodo, Todo};
use crate::Error;

#[async_trait]
pub trait TodoRepository: Sync + Send {
    async fn insert_new_todo(&self, todo: Todo) -> Result<Todo, Error>;
    async fn get_all(&self) -> Result<Vec<Todo>, Error>;
    async fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Error>;
    async fn update_by_id(&self, id: &str, todo: PartialTodo) -> Option<Error>;
    async fn delete_by_id(&self, id: &str) -> Option<Error>;
}
