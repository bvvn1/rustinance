use actix_web::{Responder, post, web};

#[post("/signup")]
pub async fn user_signup() -> impl Responder {}

pub async fn accounts_scope() -> actix_web::Scope {
    web::scope("/account").service(user_signup)
}
