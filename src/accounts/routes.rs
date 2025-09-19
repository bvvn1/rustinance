use actix_web::{HttpResponse, Responder, post, web};
use argon2::{
    Argon2, Error, PasswordHash, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use argon2::PasswordHasher;
use sqlx::{Pool, Postgres};

use crate::accounts::models::SignupRequest;

fn hash_password(pswd: &String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(pswd.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}

fn compare_hashes(input: &String, hash: &String) -> Result<bool, HttpResponse> {
    let argon2 = Argon2::default();

    let parsed_hash = match PasswordHash::new(hash) {
        Ok(e) => e,
        Err(_) => return Err(HttpResponse::InternalServerError().body("negur")),
    };

    let is_correct = argon2
        .verify_password(input.as_bytes(), &parsed_hash)
        .is_ok();

    return Ok(is_correct);
}

#[utoipa::path(
    post,
    path = "/account/signup",
    request_body = SignupRequest,
    responses(),
    tag = "Auth"
)]
#[post("/signup")]
pub async fn user_signup(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<SignupRequest>,
) -> impl Responder {
    let hashed = match hash_password(&req.password) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Hashing failed"),
    };

    match sqlx::query(
        r#"
            INSERT INTO users (username, password_hash)
            VALUES ($1, $2)
            "#,
    )
    .bind(req.username.clone())
    .bind(hashed.clone())
    .execute(db.get_ref())
    .await
    {
        Ok(e) => e,
        Err(_) => return HttpResponse::InternalServerError().body("Error during signup query"),
    };

    HttpResponse::Ok().finish()
}
#[utoipa::path(
    post,
    path = "/account/login",
    request_body = SignupRequest,
    responses(),
    tag = "Auth"
)]
#[post("/login")]
pub async fn user_login(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<SignupRequest>,
) -> impl Responder {
    let hashed = match hash_password(&req.password) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Hashing failed"),
    };

    let password_hash: Option<String> = match sqlx::query_scalar(
        r#"
                SELECT password_hash FROM users WHERE username = $1
    
    
                "#,
    )
    .bind(req.username.clone())
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(e) => e,
        Err(_) => return HttpResponse::InternalServerError().body("Error during login query"),
    };

    if let Some(password_hash) = password_hash      //Ti si halka oprai go gei (hashvaneto)
        && password_hash == hashed
    {
        return HttpResponse::Ok().finish();
    } else {
        return HttpResponse::Unauthorized().finish();
    }
}

pub fn accounts_scope() -> actix_web::Scope {
    web::scope("/account")
        .service(user_signup)
        .service(user_login)
}
