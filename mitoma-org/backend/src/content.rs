use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_web::{
    get,
    http::header::ContentType,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};
use log::{debug, info};
use pulldown_cmark::{html, Event, LinkType, Options, Tag};
use serde::{Deserialize, Serialize};

use crate::Args;

#[derive(Deserialize)]
struct ContentPath {
    content_path: String,
}

#[derive(Serialize)]
struct ContentOutput {
    pub(crate) html: String,
}

#[get("/v1/content/{content_path:.*}")]
async fn content(
    path: web::Path<ContentPath>,
    req: HttpRequest,
    args: Data<Args>,
) -> impl Responder {
    info!("path:{:?}", path.content_path);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut path_buf = PathBuf::new();
    path_buf.push(&args.contents_file_path);
    let path_buf = path_buf.canonicalize().unwrap();

    let mut md_path_buf = path_buf.clone();
    md_path_buf.push(format!("{}.md", &path.content_path));

    // ディレクトリトラバーサル対策は actix 側でされているので基本はここは通らない
    if !md_path_buf.starts_with(path_buf.clone()) {
        return HttpResponse::BadRequest().finish();
    }

    if let Ok(input) = std::fs::read_to_string(md_path_buf.as_path()) {
        let parser = pulldown_cmark::Parser::new_ext(&input, options);

        let parser = parser.map(|event| {
            let content_dir = Path::new(&path.content_path)
                .parent()
                .unwrap_or(Path::new(""))
                .to_path_buf();
            let mut new_link_buf = PathBuf::new();
            new_link_buf.push("/api/v1/content");
            new_link_buf.push(content_dir);

            let result = match event {
                Event::Start(Tag::Image(LinkType::Inline, link, title))
                    if !link.starts_with("http") =>
                {
                    debug!("replace link:{}", link);
                    new_link_buf.push(link.to_string());
                    let new_link = new_link_buf.to_str().unwrap_or_default().to_owned();
                    Event::Start(Tag::Image(LinkType::Inline, new_link.into(), title))
                }
                Event::Start(Tag::Image(LinkType::Collapsed, link, title))
                    if !link.starts_with("http") =>
                {
                    debug!("replace link:{}", link);
                    new_link_buf.push(link.to_string());
                    let new_link = new_link_buf.to_str().unwrap_or_default().to_owned();
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
        info!("200OK");
        return HttpResponse::Ok()
            .append_header(ContentType(mime::APPLICATION_JSON))
            .body(resp_string);
    }

    let mut blob_path_buf = path_buf.clone();
    blob_path_buf.push(&path.content_path);
    if let Ok(file) = NamedFile::open_async(blob_path_buf.as_path()).await {
        info!("NamedFile");
        return file.disable_content_disposition().into_response(&req);
    }

    info!("NotFound");
    HttpResponse::NotFound().finish()
}

//#[get("/v1/content/{content_path:[^:]*}:list_md")]
#[get("/v1/content/{content_path:.*}:list_md")]
async fn list(path: web::Path<ContentPath>, args: Data<Args>) -> impl Responder {
    info!("path:{:?}", path.content_path);

    let mut path_buf = PathBuf::new();
    path_buf.push(&args.contents_file_path);
    path_buf.push(&path.content_path);

    if path_buf.is_dir() {
        if let Ok(paths) = std::fs::read_dir(path_buf) {
            for path in paths {
                info!("{:?}", path);
            }
        }
        HttpResponse::Ok()
            .append_header(ContentType(mime::APPLICATION_JSON))
            .body("{}")
    } else {
        info!("NotFound");
        HttpResponse::NotFound().finish()
    }
}
