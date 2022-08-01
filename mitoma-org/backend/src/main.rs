use std::path::PathBuf;

use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    get,
    http::header::ContentType,
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use log::debug;
use pulldown_cmark::{html, Event, LinkType, Options, Tag};
use serde::{Deserialize, Serialize};

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

    // content file path
    #[clap(short, long, default_value = "../contents")]
    contents_file_path: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Box::leak(Args::parse().into());
    HttpServer::new(|| {
        let api_app = web::scope("/api").service(health).service(content);
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

#[derive(Deserialize)]
struct ContentPath {
    content_path: String,
}

#[derive(Serialize)]
struct ContentOutput {
    pub(crate) html: String,
}

#[get("/v1/content/{content_path:.*}")]
async fn content(path: web::Path<ContentPath>, req: HttpRequest) -> impl Responder {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut path_buf = PathBuf::new();
    path_buf.push("../contents");
    let mut md_path_buf = path_buf.clone().canonicalize().unwrap();
    md_path_buf.push(format!("{}.md", &path.content_path));

    // TODO ディレクトリトラバーサルのチェック
    if let Ok(input) = std::fs::read_to_string(md_path_buf.as_path()) {
        let parser = pulldown_cmark::Parser::new_ext(&input, options);

        let parser = parser.map(|event| {
            let result = match event {
                Event::Start(Tag::Image(LinkType::Inline, link, title))
                    if !link.starts_with("http") =>
                {
                    debug!("replace link:{}", link);
                    let new_link = format!("api/v1/content/{}", link);
                    Event::Start(Tag::Image(LinkType::Inline, new_link.into(), title))
                }
                Event::Start(Tag::Image(LinkType::Collapsed, link, title))
                    if !link.starts_with("http") =>
                {
                    debug!("replace link:{}", link);
                    let new_link = format!("api/v1/content/{}", link);
                    Event::Start(Tag::Image(LinkType::Collapsed, new_link.into(), title))
                }
                other => other,
            };
            debug!("event:{:?}", result);
            result
        });

        let mut html: String = String::with_capacity(input.len() * 3 / 2);
        html::push_html(&mut html, parser);
        let resp = ContentOutput { html };
        let resp_string = serde_json::to_string(&resp).unwrap();
        return HttpResponse::Ok()
            .append_header(ContentType(mime::APPLICATION_JSON))
            .body(resp_string);
    }

    let mut blob_path_buf = path_buf.canonicalize().unwrap();
    blob_path_buf.push(&path.content_path);
    if let Ok(file) = NamedFile::open_async(blob_path_buf.as_path()).await {
        return file.into_response(&req);
    }

    HttpResponse::NotFound().finish()
}
