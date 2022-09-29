use clap::Parser;

pub mod content;
pub mod health;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "backend application of mitoma.org", long_about = None)]
pub struct Args {
    // address
    #[arg(short, long, default_value = "127.0.0.1")]
    pub address: String,

    // port number
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    // static file path
    #[arg(short, long, default_value = "../frontend/build")]
    pub static_file_path: String,

    // content file path
    #[arg(short, long, default_value = "../contents")]
    pub contents_file_path: String,

    // static file path for hello.mitoma.org
    #[arg(long, default_value = "../contents_hello")]
    pub static_file_path_for_hello: String,
}
