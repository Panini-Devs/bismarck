use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;

#[tokio::main]
async fn main() {
    let database = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("database.sqlite")
            .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect");
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Coudln't migrate.");
}
