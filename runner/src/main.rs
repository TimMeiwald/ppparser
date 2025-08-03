use std::{fs, path::PathBuf};
mod data;
use clap::Parser;
use data::DataGenerator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to Grammar file you would like to generate a parser for.
    #[arg(short, long)]
    source: PathBuf,
    /// Target Directory to place generated parser code. Note this should be the directory in which you'd place the whole parser folder.
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
        Err(_) => panic!("Failed to find Source File"),
        Ok(source) => source,
    };

    let target_str = args.target.to_str().unwrap();

    let target: PathBuf = match args.target.canonicalize() {
        Ok(path) => path,
        Err(_) => match fs::create_dir_all(target_str) {
            Ok(()) => PathBuf::from(target_str),
            Err(_) => panic!("Failed to create Target Directory"),
        },
    };

    println!("Parser Name: {:?}", args.name);
    println!("Target Directory: {target:?}");
    println!("Source File: {source:?}");

    let data = DataGenerator::new(
        target.to_str().unwrap().into(),
        "./parser",
        source,
        args.name,
    );
    match data.generate_data() {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {e:?}"),
    }
}
