// use std::env::current_dir;
// use std::fs;
// use std::path::PathBuf;
use std::include_str;


// fn get_core_code_snapshot(str: &str) -> String {
//     /*Gets all lines of a file from startline to endline*/
//     let cwd = current_dir().unwrap();
//     let mut after = PathBuf::new();
//     after.push(cwd);
//     after.push("src");
//     after.push(str);
//     let contents = fs::read_to_string(after).expect("Failed to read parser core. Make sure to run 'cargo build' \nn.B. If you deleted generated_parser_core.rs without changing any other files cargo build won't rerun due to caching. Make any change and then run cargo build again");
//     let content: Vec<String> = contents.split("\n").map(str::to_string).collect();
//     let content: String = content.concat();
//     return content;
// }

pub fn embed_core() -> String {
    //let s = get_core_code_snapshot("generated_parser_core.rs");
    let str = include_str!("generated_parser_core.rs");
    println!("{:?}", str);
    return str.to_string();
}   

#[test]
fn test_embed_core(){
    let s = embed_core();
    println!("{:?}", s);
}