use axum::{extract::State, response::IntoResponse, Json};
use sqlx::{sqlite::SqliteRow, SqlitePool};

use super::Todo;
use axum::http::StatusCode;

pub async fn get_all_todos(State(db): State<SqlitePool>) -> impl IntoResponse {
    let query = Todo::select().build();

    let result: Result<Vec<SqliteRow>, sqlx::Error> = sqlx::query(&query).fetch_all(&db).await;

    // dbg!(&result);
    match result {
        Ok(_) => {
            // results.iter().map(|row| println!("{:?}", row));
            (StatusCode::OK, Json("Fetched rewards"))
        }
        Err(error) => {
            eprintln!("Error fetching todos: {}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Unexpected Error"))
        }
    }
}
