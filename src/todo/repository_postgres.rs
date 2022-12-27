use async_trait::async_trait;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::err::Error;

use super::{
    entity::{PartialTodo, Todo},
    repository::TodoRepository,
};

pub struct TodoRepositoryPostgres {
    client: Arc<Client>,
}

#[async_trait]
impl TodoRepository for TodoRepositoryPostgres {
    async fn insert_new_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let command = self
            .client
            .execute(
                "INSERT INTO todos (text, is_done) VALUES ($1::TEXT, $2::BOOL);",
                &[&todo.text, &true],
            )
            .await;
        if command.is_err() {
            return Err(Error::InternalServerError(None));
        }
        Ok(todo)
    }
    async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let mut todos: Vec<Todo> = vec![];
        let command = match self.client.query("SELECT * FROM todos", &[]).await {
            Err(_) => return Err(Error::InternalServerError(None)),
            Ok(rows) => rows,
        };
        for row in command {
            let id: i32 = row.get(0);
            todos.push(Todo {
                id: id.to_string(),
                text: row.get(1),
                is_done: row.get(2),
            })
        }
        Ok(todos)
    }
    async fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Error> {
        let command = match self
            .client
            .query(
                "SELECT * FROM todos WHERE id = $1::INT",
                &[&id.parse::<i32>().unwrap()],
            )
            .await
        {
            Err(_) => return Err(Error::InternalServerError(None)),
            Ok(rows) => rows,
        };

        if command.len() == 0 {
            return Ok(None);
        }

        let id: i32 = command[0].get(0);

        Ok(Some(Todo {
            id: id.to_string(),
            text: command[0].get(1),
            is_done: command[0].get(2),
        }))
    }
    async fn update_by_id(&self, id: &str, todo: PartialTodo) -> Option<Error> {
        let mut update_query = String::from("UPDATE todos SET ");
        let mut update_elem_count = 0;
        if todo.text.is_some() {
            update_query.push_str(" text = ");
            update_query.push('\'');
            update_query.push_str(&todo.text.unwrap());
            update_query.push('\'');
            update_elem_count += 1;
        }
        if todo.is_done.is_some() {
            if update_elem_count > 0 {
                update_query.push(',');
            }
            update_query.push_str(" is_done = ");
            update_query.push_str(&todo.is_done.unwrap().to_string());
        }
        update_query.push_str(" WHERE id = $1::INT");
        match self
            .client
            .execute(&update_query, &[&id.parse::<i32>().unwrap()])
            .await
        {
            Ok(_) => None,
            Err(_) => Some(Error::InternalServerError(None)),
        }
    }
    async fn delete_by_id(&self, id: &str) -> Option<Error> {
        let command = self
            .client
            .execute(
                "DELETE FROM todos WHERE id = $1::INT",
                &[&id.parse::<i32>().unwrap()],
            )
            .await;
        match command {
            Err(_) => Some(Error::InternalServerError(None)),
            Ok(_) => None,
        }
    }
}

pub fn new(client: Arc<Client>) -> TodoRepositoryPostgres {
    TodoRepositoryPostgres { client }
}
