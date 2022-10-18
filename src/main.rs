use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    username: String,
 }

#[post("/info")]
async fn info(info: web::Json<Info>) -> web::Json<Info> {
        web::Json(Info {
        username: info.username.clone(),
      })
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {

    HttpResponse::Ok().body(format!("Hello {name}!"))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(info).service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
