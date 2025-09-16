use actix_web::{App, HttpServer, web};

use crate::accounts::routes::accounts_scope;

pub mod accounts;
pub mod db;
pub mod transactions;

#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().service(accounts_scope()))
        .bind("127.0.0.1", 8080)?
        .run()
        .await
}
