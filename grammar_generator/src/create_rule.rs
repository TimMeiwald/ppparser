use std::u32;

use parser_core::Source;
use stack::BasicStack;
use crate::Rules;
use crate::SymbolTable;
use crate::create_symbol_table;
use crate::count_lines;

pub fn rule(source: &Source, stack: &BasicStack, symbol_table: &SymbolTable, start_index: u32){
    let source: &String = &String::from(source);
    let start = start_index;
    let mut end = 0;
    let mut counter = 0;
    let mut rule_name: String = "No Name Found".to_string();
    for i in stack{
        if i[0] == Rules::VarNameDecl as u32{
            rule_name = source[((i[1]+1) as usize)..((i[2]-1) as usize)].to_string();
        }
        else if i[0] == Rules::EndRule as u32{
            end = counter;
            break
        }
        counter += 1;
    }
    println!("Rule Name: {}", rule_name);

    //stack.print_range(&source, start, end)
    let mut placeholder_count = 0;
    for i in stack{
        match Rules::from(i[0]) {
            Rules::OrderedChoice => {}, 
            Rules::Sequence => {},
            Rules::Optional => {}
            Rules::OneOrMore => {},
            Rules::ZeroOrMore => {},
            Rules::NotPredicate => {},
            Rules::AndPredicate => {},
            Rules::Subexpression => {},
            _ => {}
        }
        placeholder_count += 1;
    }

}

// fn ordered_choice(source: &Source, stack: &BasicStack, symbol_table: &SymbolTable, start_index: u32) -> Option<String> {
//     stack
// }



#[cfg(test)]
mod tests {
    use super::*;
    use cache::MyCache4;
    use grammar_parser::grammar;
    use parser_core::Context;
    use parser_core::Rules;
    use parser_core::Source;
    use stack::BasicStack;
    use std::env;
    use std::fs::{canonicalize, read_to_string};

    #[test]
    fn test() {
        println!("{:?}", env::current_dir().unwrap());
        let path = "../parser_core/tests/Grammar.txt";
        let pathbuf = canonicalize(path).expect("If it's moved change the string above");
        let string = read_to_string(pathbuf).expect("If it's moved change the string above");
        let string2 = string.clone();
        let src_len = string.len() as u32;
        let source = Source::new(string);
        let position: u32 = 0;
        let context = Context::<MyCache4, BasicStack>::new(src_len, 45);
        let result = grammar(&context, &source, position);

        // Checks full file was parsed. 
        if result.1 != string2.len() as u32 {
            panic!(
                "Failed to parse grammar due to syntax error on Line: {:?}",
                count_lines(&string2, result.1)
            )
        } else {
            println!("Successfully parsed")
        }
        let sym_table = create_symbol_table(&context.stack.borrow(), &string2);
        assert_eq!(result, (true, src_len));
        let stack = &*context.stack.borrow();
        stack.print_range(&String::from(source), 0, 1);
        //rule(&source, stack, &sym_table, 0)



    }
}