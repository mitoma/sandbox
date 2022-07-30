use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    get, web, App, HttpResponse, HttpServer, Responder,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about = "version calucurator for git repository", long_about = None)]
struct Args {
    // port number
    #[clap(short, long, default_value = "8080")]
    port: u16,

    // static file path
    #[clap(short, long, default_value = "../frontend/build")]
    static_file_path: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let static_file_path = args.static_file_path.clone();
    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api").service(health))
            .service(
                Files::new("/", &static_file_path)
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file =
                            NamedFile::open_async(format!("{}/index.html", "../frontend/build"))
                                .await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })
                    .redirect_to_slash_directory(),
            )
    })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await
}

#[get("/v1/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
