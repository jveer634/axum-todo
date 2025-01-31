use prkorm::Table;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub enum TodoStatus {
    Created,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Table, FromRow)]
#[table_name("todo")]
#[primary_key("id")]
pub struct Todo {
    id: i32,
    title: String,
    status: TodoStatus,
}
