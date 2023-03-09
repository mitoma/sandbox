use std::{
    error::Error,
    fs::DirEntry,
    path::{Path, PathBuf},
};

use actix_files::NamedFile;
use actix_web::{
    get,
    http::header::ContentType,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};
use log::{debug, info};
use pulldown_cmark::{html, Event, HeadingLevel, LinkType, Options, Parser, Tag};
use serde::{Deserialize, Serialize};

use crate::Args;

#[derive(Deserialize)]
struct ContentPath {
    content_path: String,
}

#[derive(Serialize)]
struct ContentOutput {
    pub(crate) title: String,
    pub(crate) html: String,
}

fn md_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

#[get("/v1/content/{content_path:.*}")]
async fn content(
    path: web::Path<ContentPath>,
    req: HttpRequest,
    args: Data<Args>,
) -> impl Responder {
    info!("path:{:?}", path.content_path);

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
        let title = {
            let mut parser = pulldown_cmark::Parser::new_ext(&input, md_options());
            get_md_title(&mut parser).unwrap_or_else(|| "No title".into())
        };

        let parser = pulldown_cmark::Parser::new_ext(&input, md_options());
        let parser = parser.map(|event| {
            let content_dir = Path::new(&path.content_path)
                .parent()
                .unwrap_or_else(|| Path::new(""))
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
        let resp = ContentOutput { title, html };
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

#[derive(Serialize, Default)]
struct ListMdOutput {
    pub(crate) path: String,
    pub(crate) list: Vec<MdMetadata>,
}

#[derive(Serialize, Default)]
struct MdMetadata {
    title: String,
    path: String,
}

impl MdMetadata {
    fn parse(entry: &DirEntry) -> Result<Self, Box<dyn Error>> {
        let file_type = entry.file_type()?;
        debug!(
            "file type:{:?}, is_dir:{}, if_file:{}",
            file_type,
            file_type.is_dir(),
            file_type.is_file()
        );
        if !entry.file_type()?.is_file() {
            return Err("is not file".into());
        }
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|_| "invalid file name")?;
        if !file_name.ends_with(".md") {
            return Err("is not markdown file".into());
        }
        let content_path = file_name.strip_suffix(".md").unwrap().to_string();

        let md_string = std::fs::read_to_string(entry.path())?;
        let mut parser = pulldown_cmark::Parser::new_ext(&md_string, md_options());

        let title = get_md_title(&mut parser).unwrap_or_else(|| "No title".into());
        Ok(Self {
            title,
            path: content_path,
        })
    }
}

/// markdown から最初の h1 要素をとってくるだけ
fn get_md_title(parser: &mut Parser) -> Option<String> {
    let mut h1_flag = false;
    parser.find_map(|event| {
        debug!("event:{:?}, h1_flag:{}", event, h1_flag);
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H1, _, _)) => {
                h1_flag = true;
                None
            }
            Event::End(Tag::Heading(HeadingLevel::H1, _, _)) => {
                h1_flag = false;
                None
            }
            Event::Text(text) => {
                if h1_flag {
                    Some(text.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    })
}

#[get("/v1/content/{content_path:.*}:list_md")]
async fn list(path: web::Path<ContentPath>, args: Data<Args>) -> impl Responder {
    info!("path:{:?}", path.content_path);

    let mut path_buf = PathBuf::new();
    path_buf.push(&args.contents_file_path);
    path_buf.push(&path.content_path);

    if path_buf.is_dir() {
        let mut resp = ListMdOutput {
            path: path.content_path.trim_end_matches('/').to_string(),
            ..Default::default()
        };
        let md_meta = std::fs::read_dir(path_buf)
            .unwrap()
            .flatten()
            .flat_map(|entry| MdMetadata::parse(&entry))
            .collect::<Vec<_>>();
        resp.list = md_meta;
        let resp_string = serde_json::to_string(&resp).unwrap();
        HttpResponse::Ok()
            .append_header(ContentType(mime::APPLICATION_JSON))
            .body(resp_string)
    } else {
        info!("NotFound");
        HttpResponse::NotFound().finish()
    }
}
