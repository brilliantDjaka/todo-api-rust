use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use todo_rust::{
    mongodb,
    todo::{self, controller::controller_list},
    AppState,
};

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Serve is running 🚀")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo = mongodb::connect().await.unwrap();
    let todo_repo = todo::repository::new(mongo.clone());
    let todo_service = todo::service::new(todo_repo);
    let state = AppState { todo_service };
    let state = web::Data::new(state);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(health_check)
            .service(controller_list())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
