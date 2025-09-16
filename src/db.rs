use actix_web::http::Error;
use once_cell::sync::OnceCell;
use sqlx::{Connection, Pool, Postgres, postgres::PgPoolOptions};

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn connect_db() {
    let url = "postgres://postgres:postgres@db:5432/finance";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("error initializing pool");

    sqlx::query("DELETE FROM table")
        .execute(&pool)
        .await
        .expect("error during qeuery");

    DB_POOL.set(pool).expect("failed to connect to db")
}
