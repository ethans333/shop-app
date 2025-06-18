use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

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