use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, Responder, HttpResponse, http::header::ContentType};
use log::debug;
use pulldown_cmark::{Options, Event, Tag, LinkType, html};
use serde::{Deserialize, Serialize};

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
