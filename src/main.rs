use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// xop opens files from a shell
pub struct Args {
    /// Files to open
    #[arg(value_name = "FILES")]
    files: Vec<String>,
}

pub fn run(args: Args) -> MyResult<()> {
    println!("{:?}", args);
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
