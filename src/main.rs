use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;

use crate::accounts::routes::accounts_scope;

pub mod accounts;
pub mod transactions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "postgres://myuser:example@0.0.0.0:5432/FinanceDB";

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("error initializing pool");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users(
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    balance INTEGER,
    password_hash TEXT NOT NULL
    );",
    )
    .execute(&pool)
    .await
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %T"))
            .app_data(web::Data::new(pool.clone()))
            .service(accounts_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
