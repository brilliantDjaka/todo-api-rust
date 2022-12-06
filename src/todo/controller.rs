use rocket::{http::Status, post, routes, serde::json::Json, Route, State};

use crate::AppState;

use super::entity::Todo;

pub fn controller_list() -> Vec<Route> {
    routes![add]
}

#[post("/")]
async fn add(service: &State<AppState>) -> Result<Json<Todo>, Status> {
    match service.todo_service.add_todo("cleaning the room").await {
        Err(_) => Err(Status::InternalServerError),
        Ok(todo) => Ok(Json(todo)),
    }
}
