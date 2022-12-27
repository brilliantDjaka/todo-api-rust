use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::{env, sync::Arc};
use todo_rust::{postgres, todo, AppState};
// use todo_rust::mongodb;
#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running 🚀")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let mongo = mongodb::connect().await.unwrap();
    // let todo_repo = Arc::new(todo::repository_mongo::new(mongo.clone()));

    let postgre = Arc::new(postgres::connect().await);
    let todo_repo = Arc::new(todo::repository_postgres::new(postgre.clone()));

    let todo_service = Arc::new(todo::service::new(todo_repo));
    let state = AppState { todo_service };
    let state = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(health_check)
            .service(todo::controller::controller_list())
    })
    .bind((
        env::var("HOST").unwrap_or(String::from("127.0.0.1")),
        env::var("PORT")
            .unwrap_or(String::from("8000"))
            .parse::<u16>()
            .unwrap(),
    ))?
    .run()
    .await
}
