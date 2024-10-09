use std::cell::RefCell;
use std::fs::read_to_string;
use std::path::PathBuf;
mod binary_wo;
mod constructor;
mod symbol_table;
use crate::constructor::GeneratedCode;
use crate::symbol_table::SymbolTable;
use parser::*;

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
    let src_len = string.len();
    let source = Source::new(&string);
    let position: u32 = 0;
    let context = BasicContext::new(src_len, RULES_SIZE as usize);
    let context: RefCell<BasicContext> = context.into();
    let result = grammar(Key(0), &context, &source, position);
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
    let tree = &context.borrow();
    //tree.print(Key(0), None);
    let tree = &tree.clear_false();
    let src = &String::from(source);
    let sym_table = SymbolTable::new(tree, src);
    //sym_table.print();
    let gen_code = GeneratedCode::new(&sym_table, tree, src);
    Some(gen_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count_lines;
    use parser::*;
    use std::cell::RefCell;
    use std::env;
    use std::fs::{canonicalize, read_to_string};
    #[test]
    fn test() {
        let path = "../parser/tests/test_data/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let gen_code = generate_parser(&pathbuf);
        match gen_code {
            Some(gen_code) => {
                //gen_code.print();
                print!("{}\n", gen_code.parser_file_content());
                print!("{}\n", gen_code.rules_enum_file_content());
            }
            None => {
                panic!("Something went wrong")
            }
        }
    }
}
