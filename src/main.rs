use actix_web::{web, post, App, HttpServer, Responder, HttpResponse};
use sha256::digest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    text: String,
}
#[derive(Serialize)]
struct Response {
    hash: String
}

#[post("/")]
async fn index(data: web::Json<Request>) -> impl Responder {
    let mut res = Response {
        hash: String::from("Default Response")
    };
    res.hash = digest(data.text.clone());
    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}