use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
mod copy_dir;
mod data;
use clap::Parser;
use data::DataGenerator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to Grammar file you would like to generate a parser for.
    #[arg(short, long)]
    source: PathBuf,
    /// Target Directory to place generated parser code.
    #[arg(short, long)]
    target: PathBuf,
    /// Name of parser.
    #[arg(short, long)]
    name: String,
}

// source = "./parser_core/tests/Grammar.txt"
// target = "./generation_test"
// name of parser = "test_parser"

fn main() {
    let args = Args::parse();

    //let source: Path = args.source.canonicalize().ok_or_else(panic!("Failed to find Source File"));
    let source: PathBuf = match args.source.canonicalize() {
        Err(_e) => panic!("Failed to find Source File"),
        Ok(source) => source,
    };

    let target: PathBuf = match args.target.canonicalize() {
        Err(e) => {
            let res = std::fs::create_dir(args.target);
            return res.unwrap_or_else(|_| panic!("Failed to create Target Directory!\n{:?}", e));
        }
        Ok(target) => target,
    };

    println!("{:?}", args.name);
    println!("{:?}", target);
    println!("{:?}", source);

    let data = DataGenerator::new(
        "./cache",
        "./rules",
        "./publisher",
        "./grammar_generator",
        "./grammar_runner",
        target,
    );
    data.generate_data();
}
