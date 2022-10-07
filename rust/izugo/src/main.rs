use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use anyhow::Result;
use chrono::Local;
use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(author, version, about = "watch(1) like command.", long_about = None)]
pub(crate) struct Args {
    // Command
    #[arg(default_value = "uptime")]
    command: String,

    #[arg(long, short, default_value = "plane")]
    output: OutputFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Plane,
    Json,
}

#[derive(Serialize)]
pub(crate) struct Output {
    time: String,
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stderr: Option<String>,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {:?} (stderr:{:?})",
            self.time, self.stdout, self.stderr
        )
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    loop {
        let proc = std::process::Command::new(&args.command).output()?;
        let stdout = if proc.stdout.is_empty() {
            None
        } else {
            Some(String::from_utf8(proc.stdout)?.trim().to_string())
        };
        let stderr = if proc.stderr.is_empty() {
            None
        } else {
            Some(String::from_utf8(proc.stderr)?.trim().to_string())
        };
        let out = Output {
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            stdout,
            stderr,
        };

        match args.output {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&out)?),
            OutputFormat::Plane => println!("{}", out),
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
