use actix_web::{HttpResponse, Responder, get, http::header::ContentType};

#[get("/v1/health")]
pub(crate) async fn health() -> impl Responder {
    HttpResponse::Ok()
        .append_header(ContentType(mime::APPLICATION_JSON))
        .body(r#"{"status": "OK"}"#)
}
