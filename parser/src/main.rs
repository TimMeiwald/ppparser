use clap::Parser;
use std::path::PathBuf;
mod parse;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file you would like to parse.
    #[arg(short, long)]
    source: PathBuf,
}

fn main() {
    let args = Args::parse();

    //let source: Path = args.source.canonicalize().ok_or_else(panic!("Failed to find Source File"));
    let source: PathBuf = match args.source.canonicalize() {
        Err(_e) => panic!("Failed to find Source File"),
        Ok(source) => source,
    };
    let r = parse::parse(source);
    match r {
        Ok(result) => match result {
            true => {
                println!("Success")
            }
            false => println!("Source file is not a valid file for this parser"),
        },
        Err(_) => {
            panic!("Something went wrong");
        }
    }
}
