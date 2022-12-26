use mongodb::{bson::Document, error::Error, Client};
use std::env;
pub async fn connect() -> Result<Client, Error> {
    let client = Client::with_uri_str(
        env::var("MONGO_URI").unwrap_or(String::from("mongodb://localhost:27017/todo_db")),
    )
    .await?;

    match client.default_database() {
        None => panic!("You must spesify database on MONGODB_URI"),
        _ => "",
    };

    Ok(client)
}

pub trait EntityDoc {
    fn from_doc(doc: Document) -> Self;
    fn into_doc(&self) -> Document;
}
