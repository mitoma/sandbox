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

    // Command args
    command_args: Vec<String>,

    #[arg(long, short, default_value = "plane")]
    output: OutputFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Plane,
    Json,
    JsonPretty,
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
            "[{}] OUT:{} ERR:{}",
            self.time,
            self.stdout
                .clone()
                .unwrap_or_else(|| { "(empty)".to_string() }),
            self.stderr
                .clone()
                .unwrap_or_else(|| { "(empty)".to_string() })
        )
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    loop {
        let proc = std::process::Command::new(&args.command)
            .args(&args.command_args)
            .output()?;

        let stdout = String::from_utf8(proc.stdout)
            .map(|s| s.trim().to_string())
            .ok();
        let stderr = String::from_utf8(proc.stderr)
            .map(|s| s.trim().to_string())
            .ok();

        let out = Output {
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            stdout,
            stderr,
        };

        match args.output {
            OutputFormat::Json => println!("{}", serde_json::to_string(&out)?),
            OutputFormat::JsonPretty => println!("{}", serde_json::to_string_pretty(&out)?),
            OutputFormat::Plane => println!("{out}"),
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
