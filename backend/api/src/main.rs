use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[post("/user")]
async fn create_user(payload: web::Json<User>) -> impl Responder {
    let data = payload.into_inner();

    HttpResponse::Ok().body(format!(
        "Created new User {}",
        data.name
    ))
}

#[get("/user/{id}")]
async fn get_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("User ID is {}", user_id))
}

#[get("/info")]
async fn get_info() -> impl Responder {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "unknown".to_string());
    HttpResponse::Ok().body(format!("Database URL: {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}