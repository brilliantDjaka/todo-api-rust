use tokio;
use tokio_postgres::{Client, NoTls};

pub async fn connect() -> Client {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=todo_db",
        NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    return client;
}
