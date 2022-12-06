use rocket::{form::Form, http::Status, post, routes, serde::json::Json, FromForm, Route, State};

use crate::AppState;

use super::entity::Todo;

pub fn controller_list() -> Vec<Route> {
    routes![add]
}
#[derive(FromForm)]
pub struct AddTodoDto {
    pub text: String,
}

#[post("/", data = "<dto>")]
async fn add(dto: Form<AddTodoDto>, service: &State<AppState>) -> Result<Json<Todo>, Status> {
    match service.todo_service.add_todo(dto.into_inner()).await {
        Err(_) => Err(Status::InternalServerError),
        Ok(todo) => Ok(Json(todo)),
    }
}
