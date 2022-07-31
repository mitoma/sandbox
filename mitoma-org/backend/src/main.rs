use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    get,
    http::header::ContentType,
    web, App, HttpResponse, HttpServer, Responder,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "version calucurator for git repository", long_about = None)]
struct Args {
    // address
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,

    // port number
    #[clap(short, long, default_value = "8080")]
    port: u16,

    // static file path
    #[clap(short, long, default_value = "../frontend/build")]
    static_file_path: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Box::leak(Args::parse().into());
    HttpServer::new(|| {
        let api_app = web::scope("/api").service(health);
        let static_file_app = Files::new("/", &args.static_file_path)
            .index_file("index.html")
            .default_handler(|req: ServiceRequest| async {
                let (req, _) = req.into_parts();
                let file = NamedFile::open_async(&format!("{}/index.html", &args.static_file_path))
                    .await?;
                let res = file.into_response(&req);
                Ok(ServiceResponse::new(req, res))
            })
            .redirect_to_slash_directory();
        App::new().service(api_app).service(static_file_app)
    })
    .bind((args.address.as_str(), args.port))?
    .run()
    .await
}

#[get("/v1/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .append_header(ContentType(mime::APPLICATION_JSON))
        .body(r#"{"__html": "<b>OK</b>"}"#)
}
