use mongodb::{error::Error, Client};

pub const DB_NAME: &str = "todo_db";

pub async fn connect() -> Result<Client, Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    Ok(client)
}
