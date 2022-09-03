use clap::Parser;

pub mod content;
pub mod health;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about = "version calucurator for git repository", long_about = None)]
pub struct Args {
    // address
    #[clap(short, long, default_value = "127.0.0.1")]
    pub address: String,

    // port number
    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    // static file path
    #[clap(short, long, default_value = "../frontend/build")]
    pub static_file_path: String,

    // content file path
    #[clap(short, long, default_value = "../contents")]
    pub contents_file_path: String,

    // static file path for hello.mitoma.org
    #[clap(long, default_value = "../contents_hello")]
    pub static_file_path_for_hello: String,
}
