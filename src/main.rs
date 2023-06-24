use clap::Parser;
use std::error::Error;
use std::process::Command;
use which::which;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// xop opens files from a shell
pub struct Args {
    /// Files to open
    #[arg(value_name = "FILES", required(true))]
    files: Vec<String>,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let opener = find_opener()?;
    for file in &args.files {
        call_open(&opener, file)?;
    }
    Ok(())
}

pub fn call_open(command: &str, path: &str) -> Result<(), Box<dyn Error>> {
    Command::new(command).arg(path).spawn()?;
    Ok(())
}

pub fn find_opener() -> Result<String, Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        // TODO: add windows support
    } else if cfg!(target_os = "macos") && which("open").is_ok() {
        return Ok("open".to_string());
    } else if cfg!(target_os = "linux") {
        let candidates = vec!["xdg-open", "gnome-open", "kde-open"];
        for candidate in candidates {
            if which(candidate).is_ok() {
                return Ok(candidate.to_string());
            }
        }
    }
    Err("No opener found".into())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
