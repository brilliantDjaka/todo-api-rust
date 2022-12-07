use actix_web::{delete, get, patch, post, web, web::Form, HttpRequest, HttpResponse, Scope};
use serde::Deserialize;

use crate::err::convert_err;
use crate::AppState;

pub fn controller_list() -> Scope {
    web::scope("/todo")
        .service(get_all)
        .service(get_by_id)
        .service(delete_by_id)
        .service(add)
        .service(update_by_id)
}
#[derive(Deserialize)]
pub struct AddTodoDto {
    pub text: String,
}

#[post("")]
async fn add(dto: Form<AddTodoDto>, service: web::Data<AppState>) -> HttpResponse {
    match service.todo_service.add_todo(dto.into_inner()).await {
        Err(err) => convert_err(err).body(""),
        Ok(todo) => HttpResponse::Created().json(todo),
    }
}

#[get("")]
pub async fn get_all(service: web::Data<AppState>) -> HttpResponse {
    match service.todo_service.get_all().await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => convert_err(err).body(""),
    }
}
#[get("/{id}")]
async fn get_by_id(req: HttpRequest, service: web::Data<AppState>) -> HttpResponse {
    match service
        .todo_service
        .get_by_id(req.match_info().get("id").unwrap())
        .await
    {
        Err(err) => convert_err(err).body(""),
        Ok(todo) => match todo {
            Some(todo) => HttpResponse::Ok().json(todo),
            None => HttpResponse::NotFound().body(""),
        },
    }
}
#[derive(Deserialize)]
pub struct UpdateTodoDto {
    pub text: String,
    pub is_done: bool,
}

#[patch("/{id}")]
async fn update_by_id(
    req: HttpRequest,
    dto: Form<UpdateTodoDto>,
    service: web::Data<AppState>,
) -> HttpResponse {
    match service
        .todo_service
        .update_by_id(req.match_info().get("id").unwrap(), dto.into_inner())
        .await
    {
        Err(err) => convert_err(err).body(""),
        Ok(todo) => HttpResponse::Ok().json(todo),
    }
}

#[delete("/{id}")]
async fn delete_by_id(req: HttpRequest, service: web::Data<AppState>) -> HttpResponse {
    match service
        .todo_service
        .delete_by_id(req.match_info().get("id").unwrap())
        .await
    {
        Some(err) => convert_err(err).body(""),
        None => HttpResponse::Ok().body(""),
    }
}
