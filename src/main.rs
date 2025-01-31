mod todos;

use axum::{response::Html, routing::get, Router};
use listenfd::ListenFd;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite};
use tokio::net::TcpListener;

use todos::{get_routes, Todo};

#[tokio::main]
async fn main() {
    const PORT: i32 = 3000;
    const DATABASE_URL: &str = "sqlite://database.db";

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind(format!("127.0.0.1:{PORT}"))
            .await
            .unwrap(),
    };

    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false) {
        println!("Creating database {}", DATABASE_URL);
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Database created successfully"),
            Err(error) => panic!("Error creating database: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(DATABASE_URL)
        .await
        .unwrap();

    let app = Router::new()
        .route("/hello", get(handler))
        .merge(todos::get_routes())
        .with_state(pool);

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
