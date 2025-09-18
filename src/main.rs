use actix_web::{App, HttpServer, web};
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{accounts::routes::accounts_scope, db::connect_db};

pub mod accounts;
pub mod transactions;

#[actix_web::main]
async fn main() {
    let url = "postgres://postgres:postgres@db:5432/finance";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("error initializing pool");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(accounts_scope())
    })
    .bind("127.0.0.1", 8080)?
    .run()
    .await
}
