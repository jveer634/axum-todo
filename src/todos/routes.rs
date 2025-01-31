use axum::{routing::get, Router};
use sqlx::{Pool, Sqlite};

use super::get_all_todos;

pub fn get_routes() -> Router<Pool<Sqlite>> {
    let router = Router::new().route("/", get(get_all_todos));
    router
}
