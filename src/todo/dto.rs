use serde::Deserialize;
use validator::Validate;

use crate::{todo::entity::PartialTodo, ValidateRequest};

#[derive(Deserialize)]
pub struct AddTodoDto {
    pub text: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTodoDto {
    #[validate(length(min = 1))]
    pub text: Option<String>,
    pub is_done: Option<bool>,
}
impl UpdateTodoDto {
    pub fn into_partial_todo(&self) -> PartialTodo {
        PartialTodo {
            id: None,
            text: self.text.to_owned(),
            is_done: self.is_done,
        }
    }
}
impl ValidateRequest for UpdateTodoDto {}
