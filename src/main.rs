use todo_rust::{mongodb, todo, AppState};

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let mongo = mongodb::connect().await.unwrap();
    let todo_repo = todo::repository::new(mongo.clone());
    let todo_service = todo::service::new(todo_repo);

    let state = AppState { todo_service };

    rocket::build()
        .manage(state)
        .mount("/todo", todo::controller::controller_list())
}
