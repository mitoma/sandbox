use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use backend::{
    content::{content, list},
    health::health,
    Args,
};
use clap::Parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Box::leak(Args::parse().into());
    HttpServer::new(|| {
        // API 向けに /api は使う
        let api_app = web::scope("/api")
            .service(health)
            .service(list)
            .service(content)
            .app_data(Data::new(args.clone())) // 起動引数は app_data として各コントローラーで参照可能にする;
            .wrap(Logger::default());

        // 静的コンテンツ向けに / は static_file_path を見る
        let static_file_app = Files::new("/", &args.static_file_path)
            .index_file("index.html")
            .default_handler(|req: ServiceRequest| async {
                // SPA なのでパスに対応するコンテンツがない場合は基本的に index.html を返す
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
