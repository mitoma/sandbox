use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

#[get("/v1/health")]
pub(crate) async fn health() -> impl Responder {
    HttpResponse::Ok()
        .append_header(ContentType(mime::APPLICATION_JSON))
        .body(r#"{"status": "OK"}"#)
}
