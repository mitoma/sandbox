use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "watch(1) like command.", long_about = None)]
pub(crate) struct Args {
    // Command
    #[arg(default_value = "uptime")]
    command: String,
}

fn main() {
    let args = Args::parse();
    let proc = std::process::Command::new(args.command).output().unwrap();
    let stdout = if proc.stdout.is_empty() {
        None
    } else {
        Some(String::from_utf8(proc.stdout).unwrap())
    };
    let stderr = if proc.stderr.is_empty() {
        None
    } else {
        Some(String::from_utf8(proc.stderr).unwrap())
    };

    if let Some(out) = stdout {
        println!("STDOUT:{}", out);
    }
    if let Some(out) = stderr {
        println!("STDERR:{}", out);
    }
}
