use std::fs::read_to_string;
use std::path::PathBuf;
mod binary_wo;
mod constructor;
mod symbol_table;
use crate::constructor::GeneratedCode;
use crate::symbol_table::SymbolTable;
use cache::MyCache4;
use grammar_parser::grammar;
use grammar_parser::Context;
use grammar_parser::Source;
use publisher::Publisher;
use publisher::Tree;
use rules::Key;
fn count_lines(source: &String, start_position: u32) -> u32 {
    let mut new_line_count: u32 = 1;

    for i in &source.as_bytes()[0..start_position as usize] {
        if *i == b'\n' {
            new_line_count += 1;
        }
    }
    new_line_count
}

pub fn generate_parser(source: &PathBuf) -> Option<GeneratedCode> {
    let string = read_to_string(source).expect("If it's moved change the string above");
    let string2 = string.clone();
    let src_len = string.len() as u32;
    let source = Source::new(string);
    let position: u32 = 0;
    let context = Context::<MyCache4, Tree>::new(src_len, 52);
    let result = grammar(&context, &source, position);
    // let tree = &context.stack.borrow();
    // tree.print(Key(0), Some(true));
    // println!("Grammar Parsing Result: {} {}", result.0, result.1);
    // Checks full file was parsed.
    if result.1 != string2.len() as u32 {
        println!(
            "Failed to parse grammar due to syntax error on Line: {:?}",
            count_lines(&string2, result.1)
        );
        return None;
    } else {
        println!("Successfully parsed")
    }
    let tree = &context.stack.borrow();
    //tree.print(Key(0), None);
    let tree = &tree.clear_false();
    let src = &String::from(source);
    let sym_table = SymbolTable::new(tree, src);
    //sym_table.print();
    let gen_code = GeneratedCode::new(&sym_table, &tree, src);
    return Some(gen_code);
}
