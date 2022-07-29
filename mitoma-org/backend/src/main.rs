use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/v1/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").service(health)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
