mod utils;
use grammar_parser::{grammar, Context, Source};
use std::env::args;
use std::fs;
use std::process::ExitCode;

use utils::*;
// use parser::output_stack::Stack;
// use parser::cache::Cache;

// fn amain() {
//     //let args: Vec<String> = env::args().collect().;
//     //dbg!(&args);
//     let grammar_path: String = args()
//         .nth(1)
//         .unwrap_or("There is no grammar filepath!".to_string());
//     let dest_path: String = args()
//         .nth(2)
//         .unwrap_or("There is no destination filepath!".to_string());

//     let grammar_file = fs::canonicalize(&grammar_path);
//     let grammar_string = match grammar_file {
//         Ok(grammar_file) => grammar_file,
//         Err(error) => {
//             println!("Error finding grammar file location: {:?}", &grammar_path);
//             println!("{}", error);
//             std::process::exit(1);
//         }
//     };
//     //println!("Source Grammar File: {:?}", grammar);

//     // Fails if file does not exist. Need to create file first to prevent it failing.
//     let dest_file = fs::canonicalize(&dest_path);
//     let _dest = match dest_file {
//         Ok(dest_file) => dest_file,
//         Err(error) => {
//             println!("Error finding dest file location: {:?}", &dest_path);
//             println!("{}", error);
//             std::process::exit(2);
//         }
//     };
//     //println!("Destination File: {:?}", dest);
//     //let core_parser: String = embed_core();
//     let grammar_string = read_grammar(grammar_string);
//     //  Temp ignore unused code with _grammmar since it's needed on line 39 later
//     let grammar_string = match grammar_string {
//         Ok(grammar_string) => grammar_string,
//         Err(_) => {
//             println!(
//                 "Could not read grammar file at destination {:?}",
//                 &grammar_path
//             );
//             std::process::exit(3);
//         }
//     };

//     let position = 0;
//     let source = Source::new(grammar_string);
//     let grammar_length = source.get_len();
//     //use std::time::Instant;
//     //let now = Instant::now();
//     let (bool, position) = grammar(&source, position);
//     //let elapsed = now.elapsed();
//     //println!("Elapsed: {:.2?}", elapsed);

//     if !bool{
//         println!("Failed to parse, result was false");
//         std::process::exit(5);
//     }
//     if position != grammar_length {
//         println!("Failed to parse, Source file length is {}, Parser only reached {}.", grammar_length-2, position);
//         std::process::exit(6);
//     }
// }

fn main() -> ExitCode {
    // Ensures scope change kills everything

    // Handwritten means that alphabet upper and lower were replaced with obvious handwritten code.
    // No cache debug 346 lines a second on Grammar.txt
    // No cache release 1040 lines a second on Grammar.txt
    // No cache 2000 los merely by replacing alphabet upper and lower with a more obvious handwritten code
    // No cache 5600 los merely by replacing alphabet upper and lower with a more obvious handwritten code -> Definitely need's an optimization pass on generation to minimize terminal calls for large terminal option blocks
    // Regular Cache, Debug, No handwritten 1083 los.
    // Regular Cache, No handwritten 9300 los.
    // Regular Cache, handwritten 28000 los(handwritten not cached since 2 comparisons are likely faster than cache access for a single char). ?? What did I change -> May well have failed to parse hence being "fast"
    //let args: Vec<String> = env::args().collect().;
    //dbg!(&args);
    let grammar_path: String = args().nth(1).expect("There is no grammar filepath!");
    // let _dest_path: String = args()
    //     .nth(2)
    //     .unwrap_or("There is no destination filepath!".to_string());

    let grammar_file = fs::canonicalize(&grammar_path);
    let grammar_string = match grammar_file {
        Ok(grammar_file) => grammar_file,
        Err(error) => {
            println!("Error finding grammar file location: {:?}", &grammar_path);
            println!("{}", error);
            std::process::exit(1);
        }
    };

    //let core_parser: String = embed_core();
    let grammar_string = read_grammar(grammar_string);
    //  Temp ignore unused code with _grammmar since it's needed on line 39 later
    let grammar_string = match grammar_string {
        Ok(grammar_string) => grammar_string,
        Err(_) => {
            println!(
                "Could not read grammar file at destination {:?}",
                &grammar_path
            );
            std::process::exit(3);
        }
    };
    // let src_len = grammar_string.len() as u32;
    // let context = Context::new(src_len, 42);
    use std::time::Instant;
    let src_len = grammar_string.len() as u32;
    let total = Instant::now();

    //let mut now = Instant::now();
    let context = Context::new(src_len, 42);

    let position = 0;

    let source = Source::new(grammar_string);

    for _i in 0..1000 {
        let parse_time = Instant::now();
        let (bol, _position) = grammar(&context, &source, position);
        println!("Parse time elapsed: {:.2?}", parse_time.elapsed());

        assert!(bol); //-> To test it actually parsed correctly
        assert_eq!(_position, src_len); //
        let cache_time = Instant::now();
        context.clear_cache();
        println!("Cache time elapsed: {:.2?}", cache_time.elapsed())
        //let elapsed = now.elapsed();
        //println!("Elapsed with file read: {:.2?}", elapsed);
        //now = Instant::now();

        //println!("{:?}, {:?}", bol, _position);

        //println!("{:?}", i)
    }
    // let (bol, _position) = grammar(&context, &source, position);
    // println!("{:?}, {:?}", bol, _position);

    // assert_eq!(bol, true); //-> To test it actually parsed correctly
    // assert_eq!(_position, src_len); //

    //34567 lines per second no impl
    //34355 lines per second impl in Kernels
    //98245 lines per second impl everywhere
    //35K with btreemap cache
    //10K with MyCache1 but allocating after timer
    //260K with MyCache1 but allocating before timer // Really drops off with larger cache sizes unsuprisingly. Cannot use LRU though if I want to support Left Recursion.
    //210K MyCache2 maybe due to less easily simd. - May have just been one off
    //243K with new change. Which is odd.
    // 310K using map instead of for loop on MyCache2
    let elapsed = total.elapsed();
    println!("Elapsed with file read: {:.2?}", elapsed);
    //println!("Lines a Second: {:?}", (52*100)/elapsed.as_secs());
    println! {"Exiting main"}
    ExitCode::from(0)
}
