use ppparser::utils::{embed_core, read_grammar, write_parser};
use std::collections::btree_map::Range;
use std::env;
use std::fs;
use ppparser::parser::Grammar;
use ppparser::parser::Resolvable;
use std::process::ExitCode;

fn amain(){
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let grammar_file = fs::canonicalize(&args[1]);
    let grammar = match grammar_file {
        Ok(grammar_file) => grammar_file,
        Err(error) => {
            println!("Error finding grammar file location: {:?}", &args[1]);
            println!("{}", error);
            std::process::exit(1);
        }
    };
    //println!("Source Grammar File: {:?}", grammar);

    // Fails if file does not exist. Need to create file first to prevent it failing.
    let dest_file = fs::canonicalize(&args[2]);
    let dest = match dest_file {
        Ok(dest_file) => dest_file,
        Err(error) => {
            println!("Error finding dest file location: {:?}", &args[2]);
            println!("{}", error);
            std::process::exit(2);
        }
    };
    //println!("Destination File: {:?}", dest);
    let core_parser: String = embed_core();
    let grammar = read_grammar(grammar);
    //  Temp ignore unused code with _grammmar since it's needed on line 39 later
    let _grammar = match grammar {
        Ok(grammar) => grammar,
        Err(_) => {
            println!("Could not read grammar file at destination {:?}", &args[1]);
            std::process::exit(3);
        }
    };
    let _grammar = _grammar + "\0";
    let position = 0;
    let (bool, position) = Grammar.resolve(position, &_grammar);
    println!{"{:?}, {:?}", bool, position};

    // Add the parser generation here to then add into parser write
    let parser_write = write_parser(dest, &core_parser);
    match parser_write {
        Ok(parser_write) => parser_write,
        Err(_) => {
            println!("Could not write out parser to destination {:?}", &args[2]);
            std::process::exit(4);
        }
    };
}

fn main() -> ExitCode {
    // Ensures scope change kills everything
    for i in 1..100{
        amain();
        //println!("{:?}", i)
    }
    println!{"Exiting main"}
    return ExitCode::from(0);
}