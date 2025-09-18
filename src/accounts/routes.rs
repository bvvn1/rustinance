use actix_web::{HttpResponse, Responder, post, web};
use argon2::{
    Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};

use argon2::PasswordHasher;
use sqlx::{PgPool, Pool, Postgres};

use crate::accounts::models::SignupRequest;

fn hash_password(pswd: &String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(pswd.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}

#[post("/signup")]
pub async fn user_signup(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<SignupRequest>,
) -> impl Responder {
    let hashed = match hash_password(&req.password) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Hashing failed"),
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        "#,
        req.username,
        hashed
    )
    .execute(db.get_ref())
    .await;

    Ok(())
}

pub async fn accounts_scope() -> actix_web::Scope {
    web::scope("/account").service(user_signup)
}
