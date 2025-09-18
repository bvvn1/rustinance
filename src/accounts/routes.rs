use actix_web::{Responder, post, web};
use sqlx::{PgPool, Pool, Postgres};

#[post("/signup")]
pub async fn user_signup(db: web::Data<Pool<Postgres>>) -> impl Responder {}

pub async fn accounts_scope() -> actix_web::Scope {
    web::scope("/account").service(user_signup)
}
