use ppparser::parser::Grammar;
use ppparser::parser::Resolvable;
use ppparser::utils::{embed_core, read_grammar, write_parser};
use std::env::args;
use std::fs;
use std::process::ExitCode;
use ppparser::output_stack::Stack;
use ppparser::cache::Cache;

fn amain() {
    //let args: Vec<String> = env::args().collect().;
    //dbg!(&args);
    let grammar_path: String = args()
        .nth(1)
        .unwrap_or("There is no grammar filepath!".to_string());
    let dest_path: String = args()
        .nth(2)
        .unwrap_or("There is no destination filepath!".to_string());

    let grammar_file = fs::canonicalize(&grammar_path);
    let grammar = match grammar_file {
        Ok(grammar_file) => grammar_file,
        Err(error) => {
            println!("Error finding grammar file location: {:?}", &grammar_path);
            println!("{}", error);
            std::process::exit(1);
        }
    };
    //println!("Source Grammar File: {:?}", grammar);

    // Fails if file does not exist. Need to create file first to prevent it failing.
    let dest_file = fs::canonicalize(&dest_path);
    let dest = match dest_file {
        Ok(dest_file) => dest_file,
        Err(error) => {
            println!("Error finding dest file location: {:?}", &dest_path);
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
            println!(
                "Could not read grammar file at destination {:?}",
                &grammar_path
            );
            std::process::exit(3);
        }
    };
    let _grammar = _grammar + "\0";
    let _grammar_length = _grammar.len() as u32 + 1;
    let position = 0;
    let mut cache = Cache::new(_grammar_length, 43);
    let mut stack = Stack::new(100,100);

    let (_bool, _position) = Grammar.resolve(&mut stack, &mut cache, position, &_grammar);
    //println!("{bool}, {position}");
    //println!{"{:?}, {:?}", bool, position};

    // Add the parser generation here to then add into parser write
    let parser_write = write_parser(dest, &core_parser);
    match parser_write {
        Ok(parser_write) => parser_write,
        Err(_) => {
            println!("Could not write out parser to destination {:?}", &dest_path);
            std::process::exit(4);
        }
    };
}

fn main() -> ExitCode {
    // Ensures scope change kills everything
    use std::time::Instant;
    let now = Instant::now();

    // Handwritten means that alphabet upper and lower were replaced with obvious handwritten code.
    // No cache debug 346 lines a second on Grammar.txt
    // No cache release 1040 lines a second on Grammar.txt
    // No cache 2000 los merely by replacing alphabet upper and lower with a more obvious handwritten code
    // No cache 5600 los merely by replacing alphabet upper and lower with a more obvious handwritten code -> Definitely need's an optimization pass on generation to minimize terminal calls for large terminal option blocks
    // Regular Cache, Debug, No handwritten 1083 los.
    // Regular Cache, No handwritten 9300 los.
    // Regular Cache, handwritten 28000 los(handwritten not cached since 2 comparisons are likely faster than cache access for a single char). ?? What did I change -> May well have failed to parse hence being "fast"
    //for _i in 1..1000 {
    amain();
    //println!("{:?}", i)
    //}
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    //println!("Lines a Second: {:?}", (52*100)/elapsed.as_secs());
    println! {"Exiting main"}
    return ExitCode::from(0);
}
