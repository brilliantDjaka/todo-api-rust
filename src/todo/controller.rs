use rocket::{
    delete, form::Form, get, http::Status, patch, post, routes, serde::json::Json, FromForm, Route,
    State,
};

use crate::err::convert_err;
use crate::AppState;

use super::entity::Todo;

pub fn controller_list() -> Vec<Route> {
    routes![add, get_all, get_by_id, update_by_id, delete_by_id]
}
#[derive(FromForm)]
pub struct AddTodoDto {
    pub text: String,
}

#[post("/", data = "<dto>")]
async fn add(dto: Form<AddTodoDto>, service: &State<AppState>) -> Result<Json<Todo>, Status> {
    match service.todo_service.add_todo(dto.into_inner()).await {
        Err(err) => Err(convert_err(err)),
        Ok(todo) => Ok(Json(todo)),
    }
}

#[get("/")]
async fn get_all(service: &State<AppState>) -> Result<Json<Vec<Todo>>, Status> {
    match service.todo_service.get_all().await {
        Err(err) => Err(convert_err(err)),
        Ok(todo) => Ok(Json(todo)),
    }
}
#[get("/<id>")]
async fn get_by_id(id: &str, service: &State<AppState>) -> Result<Json<Todo>, Status> {
    match service.todo_service.get_by_id(id).await {
        Err(err) => Err(convert_err(err)),
        Ok(todo) => match todo {
            Some(todo) => Ok(Json(todo)),
            None => Err(Status::NotFound),
        },
    }
}
#[derive(FromForm)]
pub struct UpdateTodoDto {
    pub text: String,
    pub is_done: bool,
}

#[patch("/<id>", data = "<dto>")]
async fn update_by_id(
    id: &str,
    dto: Form<UpdateTodoDto>,
    service: &State<AppState>,
) -> Result<Json<Todo>, Status> {
    match service
        .todo_service
        .update_by_id(id, dto.into_inner())
        .await
    {
        Err(err) => Err(convert_err(err)),
        Ok(todo) => Ok(Json(todo)),
    }
}

#[delete("/<id>")]
async fn delete_by_id(id: &str, service: &State<AppState>) -> Status {
    match service.todo_service.delete_by_id(id).await {
        Some(err) => convert_err(err),
        None => Status::Ok,
    }
}
